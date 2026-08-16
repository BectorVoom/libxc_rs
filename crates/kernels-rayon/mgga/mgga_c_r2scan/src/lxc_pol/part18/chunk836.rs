//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 836/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk836(t6134: f64, t8820: f64, t360: f64, t277: f64, t3216: f64, t495: f64, t3016: f64, t3055: f64, t537: f64, t2124: f64, t2551: f64, t2892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8821 = t8820 * t6134;
    let t8822 = t360 * t8821;
    let t8825 = t277 * t3216;
    let t8826 = t8825 * t495;
    let t8827 = t360 * t8826;
    let t8832 = t277 * t3016;
    let t8833 = t8832 * t495;
    let t8834 = t360 * t8833;
    let t8837 = t537 * t3055;
    let t8839 = t2124 * t8837 * t2551;
    let t8842 = t537 * t2892;
    (t8821, t8822, t8825, t8826, t8827, t8832, t8833, t8834, t8837, t8839, t8842)
}
