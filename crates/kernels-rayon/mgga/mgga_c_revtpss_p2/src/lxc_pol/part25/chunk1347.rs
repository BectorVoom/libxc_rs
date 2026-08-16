//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1347/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1347(t114: f64, t94974: f64, t94976: f64, t94979: f64, t94981: f64, t94983: f64, t94986: f64, t94988: f64, t1312: f64, t10259: f64, t2371: f64, t25805: f64, t28025: f64, t670: f64, t6985: f64, t92719: f64, t92737: f64, t94947: f64, t94956: f64, t94958: f64, t94960: f64, t94962: f64, t94964: f64, t94966: f64, t94968: f64, t94970: f64, t94972: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t94991 = piecewise3(t115, 0.0_f64, -t94974 - 11.0_f64 / 3.0_f64 * t94976 - 2.0_f64 * t94979 + t94981 - 3.0_f64 / 4.0_f64 * t94983 + 3.0_f64 / 4.0_f64 * t94986 - t94988 / 8.0_f64);
    let t94993 = 2.0_f64 * t1312 * t94991;
    let t94994 = 2.0_f64 * t10259 * t6985 + 6.0_f64 * t2371 * t25805 + 6.0_f64 * t2371 * t28025 + 6.0_f64 * t670 * t92737 + t92719 + 6.0_f64 * t94947 + t94956 + t94958 + t94960 + t94962 + t94964 + t94966 + t94968 + t94970 + t94972 + t94993;
    (t94991, t94994)
}
