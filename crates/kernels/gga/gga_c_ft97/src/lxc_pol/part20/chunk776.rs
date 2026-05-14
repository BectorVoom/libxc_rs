//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 776/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk776<F: Float>(t24569: F, t2601: F, t10007: F, t10157: F, t24507: F, t265: F, t2409: F, t6074: F, t2599: F, t2574: F, t6079: F, t773: F, t24460: F, t1882: F, t6081: F, t6090: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24570 = t24569 * t2601;
    let t24571 = t10007 * t24570;
    let t24575 = t10157 * t265 * t24507;
    let t24578 = t6074 * t2409;
    let t24579 = t2599 * t24578;
    let t24583 = t2574 * t773 * t6079;
    let t24587 = t2574 * t265 * t24460;
    let t24590 = t1882 * t6081;
    let t24592 = t1882 * t6090;
    (t24570, t24571, t24575, t24578, t24579, t24583, t24587, t24590, t24592)
}
