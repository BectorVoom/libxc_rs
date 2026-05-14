//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 341/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk341<F: Float>(t2574: F, t265: F, t6079: F, t1424: F, t729: F, t773: F, t766: F, t762: F, t6061: F, t1445: F, t681: F, t89: F, t1456: F, t684: F, t724: F, t713: F) -> (F, F, F, F, F, F, F, F) {
    let t6081 = t2574 * t265 * t6079;
    let t6085 = t729 * t773 * t1424;
    let t6088 = t1424 * t766;
    let t6090 = t729 * t762 * t6088;
    let t6094 = t729 * t265 * t6061;
    let t6099 = t89 * t681 * t1445 / 9.0;
    let t6101 = t724 * t1456 * t684;
    let t6105 = t729 * t1456 * t713;
    (t6081, t6085, t6088, t6090, t6094, t6099, t6101, t6105)
}
