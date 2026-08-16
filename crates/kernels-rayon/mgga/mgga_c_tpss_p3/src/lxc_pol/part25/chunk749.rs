//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 749/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk749(t345: f64, t4988: f64, t947: f64, t242: f64, t4977: f64, t2762: f64, t4826: f64, t4830: f64, t970: f64, t4834: f64, t2652: f64, t2670: f64, t2722: f64, t2731: f64, t2740: f64, t3917: f64, t3942: f64, t3970: f64, t4966: f64, t4970: f64, t4974: f64, t4980: f64, t4985: f64, t925: f64, t946: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4989 = t4988 * t345;
    let t4990 = t947 * t4989;
    let t4991 = t242 * t4990;
    let t4994 = t4977 * t345;
    let t4995 = t947 * t4994;
    let t4996 = t242 * t4995;
    let t5000 = t2762 * t4826;
    let t5001 = t242 * t5000;
    let t5004 = t970 * t4830;
    let t5005 = t242 * t5004;
    let t5008 = t970 * t4834;
    let t5009 = t242 * t5008;
    let t5012 = -t2670 + t3917 / 432.0_f64 + t925 * t4966 / 216.0_f64 - t925 * t4970 / 144.0_f64 + t925 * t4974 / 288.0_f64 + t2722 * t4980 / 1536.0_f64 + t3942 / 2304.0_f64 + t2740 * t4985 / 2304.0_f64 + t946 * t4991 / 3072.0_f64 - t2731 * t4996 / 3072.0_f64 - t2652 + t3970 / 3456.0_f64 + 5.0_f64 / 13824.0_f64 * t967 * t5001 - t967 * t5005 / 2304.0_f64 + t967 * t5009 / 4608.0_f64;
    (t4989, t4991, t4994, t4996, t5001, t5005, t5009, t5012)
}
