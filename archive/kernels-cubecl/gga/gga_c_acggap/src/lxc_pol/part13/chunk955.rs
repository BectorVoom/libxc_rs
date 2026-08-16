//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 955/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk955<F: Float>(t31720: F, t2082: F, t31289: F, t2109: F, t7780: F, t1980: F, t31028: F, t7476: F, t1988: F, t7701: F, t7705: F, t1982: F, t2015: F) -> (F, F, F, F, F, F, F) {
    let t31721 = F::cast_from(0.94344276868812456204e-3_f64) * t31720;
    let t31750 = t31289 * t2082;
    let t31751 = F::cast_from(0.13505315707191967146e-1_f64) * t31750;
    let t31752 = t7780 * t2109;
    let t31759 = t1980 * t7476 * t31028;
    let t31761 = t1988 * t7701;
    let t31763 = t1988 * t7705;
    let t31773 = t2015 * t1982;
    (t31721, t31751, t31752, t31759, t31761, t31763, t31773)
}
