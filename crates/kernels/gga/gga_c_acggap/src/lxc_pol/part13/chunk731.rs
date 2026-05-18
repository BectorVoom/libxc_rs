//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 731/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk731<F: Float>(t7698: F, t1089: F, t2090: F, t3201: F, t598: F, t1083: F, t7533: F, t1459: F, t7458: F, t7486: F, t1980: F, t2117: F, t377: F) -> (F, F, F, F, F, F, F, F) {
    let t7699 = F::new(0.42874018118069736972e-3) * t7698;
    let t7701 = t1089 * t3201 * t2090;
    let t7702 = t598 * t7701;
    let t7705 = t1089 * t1083 * t7533;
    let t7706 = t598 * t7705;
    let t7709 = t7458 * t1459 * t7486;
    let t7710 = t1980 * t7709;
    let t7712 = t377 * t2117;
    (t7699, t7701, t7702, t7705, t7706, t7709, t7710, t7712)
}
