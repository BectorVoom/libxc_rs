//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 616/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk616(t1145: f64, t2893: f64, t1144: f64, t1149: f64, t1158: f64, t1172: f64, t2821: f64, t2825: f64, t2829: f64, t2831: f64, t2834: f64, t2838: f64, t2842: f64, t2847: f64, t2853: f64, t2859: f64, t2862: f64, t2868: f64, t2869: f64, t2875: f64, t2877: f64, t2881: f64, t2890: f64) -> (f64, f64) {
    let t2894 = t1145 * t2893;
    let t2897 = -16.0_f64 / 9.0_f64 * t2821 * t2825 + 16.0_f64 / 9.0_f64 * t2829 * t2831 - 16.0_f64 / 3.0_f64 * t2834 * t2825 + 16.0_f64 / 3.0_f64 * t2838 * t2831 + 44.0_f64 / 27.0_f64 * t1172 * t2842 + 44.0_f64 / 27.0_f64 * t1158 * t2842 - 32.0_f64 / 81.0_f64 * t2847 * t2853 - 16.0_f64 / 27.0_f64 * t1158 * t2859 - 32.0_f64 / 81.0_f64 * t2862 * t2853 - 16.0_f64 / 27.0_f64 * t1172 * t2859 + 15.0_f64 * t2868 * t1145 * t2869 + 21.0_f64 * t2875 * t2877 + 3.0_f64 * t2881 * t2877 + 3.0_f64 * t1144 * t2890 - 3.0_f64 * t1149 * t2894;
    (t2894, t2897)
}
