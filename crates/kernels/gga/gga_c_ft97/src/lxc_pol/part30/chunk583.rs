//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 583/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk583<F: Float>(t2469: F, t6940: F, t242: F, t27981: F, t1882: F, t6914: F, t1131: F, t6187: F, t729: F, t762: F, t6923: F, t1424: F, t4005: F, t6194: F, t258: F, t6837: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28100 = t2469 * t6940;
    let t28102 = t242 * t27981;
    let t28106 = t1882 * t6914;
    let t28108 = t6187 * t1131;
    let t28110 = t729 * t762 * t28108;
    let t28113 = t1882 * t6923;
    let t28116 = t729 * t4005 * t1424;
    let t28120 = t729 * t6194 * t1131;
    let t28123 = t258 * t6837;
    (t28100, t28102, t28106, t28108, t28110, t28113, t28116, t28120, t28123)
}
