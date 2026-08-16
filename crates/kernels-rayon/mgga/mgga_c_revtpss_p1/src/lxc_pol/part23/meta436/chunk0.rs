//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1846/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1846(t18950: f64, t923: f64, t18909: f64, t2908: f64, t141: f64, t18913: f64, t11341: f64, t18904: f64, t18926: f64, t930: f64, t18930: f64, t15169: f64, t15170: f64, t15189: f64, t15192: f64, t15198: f64, t18944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18951 = t923 * t18950;
    let t18960 = t2908 * t18909;
    let t18961 = t141 * t18960;
    let t18963 = t2908 * t18913;
    let t18964 = t141 * t18963;
    let t18966 = t11341 * t18904;
    let t18967 = t141 * t18966;
    let t18969 = t930 * t18926;
    let t18970 = t141 * t18969;
    let t18972 = t930 * t18930;
    let t18973 = t141 * t18972;
    let t18977 = 0.60385e0_f64 * t18944 + 0.16557e0_f64 * t18961 - 0.5519e-1_f64 * t18964 - 0.36793333333333333333e-1_f64 * t18967 - 0.49671e0_f64 * t18970 + 0.33114e0_f64 * t18973 - t15169 + 0.36793333333333333333e-1_f64 * t15170 - 0.26837777777777777779e0_f64 * t15189 + t15192 + t15198;
    (t18951, t18960, t18961, t18963, t18964, t18966, t18967, t18969, t18970, t18972, t18973, t18977)
}
