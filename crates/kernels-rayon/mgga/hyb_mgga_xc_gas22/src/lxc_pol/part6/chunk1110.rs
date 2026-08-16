//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1110/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1110(t10898: f64, t10913: f64, t6969: f64, t7016: f64, t9008: f64, t9134: f64, t950: f64, t4247: f64, t7025: f64, t952: f64, t3490: f64, t3496: f64) -> (f64, f64, f64, f64, f64) {
    let t10914 = -t7016 + 4.0_f64 / 9.0_f64 * t6969 + 8.0_f64 / 9.0_f64 * t9008 - t9134 - t10898 / 3.0_f64 + t10913;
    let t10915 = t950 * t10914;
    let t10921 = t7025 * t4247;
    let t10922 = t10921 * t952;
    let t10924 = t3496 * t3490;
    (t10914, t10915, t10921, t10922, t10924)
}
