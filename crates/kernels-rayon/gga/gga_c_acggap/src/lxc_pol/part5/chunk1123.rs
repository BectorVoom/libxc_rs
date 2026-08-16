//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1123/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1123(t1396: f64, t1402: f64, t1403: f64, t1407: f64, t150: f64, t155: f64, t1828: f64, t1832: f64, t19911: f64, t19912: f64, t19976: f64, t19992: f64, t20006: f64, t20028: f64, t20041: f64, t20056: f64, t403: f64, t4099: f64, t4818: f64, t5050: f64, t5060: f64, t5065: f64, t5070: f64, t5073: f64, t5076: f64, t519: f64, t521: f64, t6039: f64, t6045: f64, t6052: f64, t6062: f64, t839: f64, t917: f64, t926: f64) -> f64 {
    let t20084 = 240.0_f64 * t1402 * t5065 * t4818 + 6.0_f64 * t5050 * t521 - (t19911 + t19912 + t19976 + t19992 + t20006 + t20028 + t20041 + t20056) * t150 * t155 + 6.0_f64 * t6039 * t403 - 12.0_f64 * t917 * t1832 + 60.0_f64 * t1402 * t6052 * t839 + 6.0_f64 * t519 * t5076 + 12.0_f64 * t1396 * t1407 - 24.0_f64 * t1402 * t1403 * t4099 - 48.0_f64 * t6045 * t5070 - 24.0_f64 * t5060 * t6062 + 3.0_f64 * t1828 * t926 - 24.0_f64 * t6045 * t5073;
    t20084
}
