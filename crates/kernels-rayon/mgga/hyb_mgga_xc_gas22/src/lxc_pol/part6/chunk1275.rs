//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1275/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1275(t10013: f64, t10022: f64, t10043: f64, t1211: f64, t1947: f64, t1955: f64, t1959: f64, t1976: f64, t19975: f64, t19990: f64, t23488: f64, t27474: f64, t27530: f64, t27571: f64, t3072: f64, t3876: f64, t3881: f64, t6088: f64, t6096: f64, t616: f64, t72: f64, t8061: f64, t8074: f64, t8077: f64, t8102: f64, t8103: f64, t9999: f64) -> f64 {
    let t27575 = -t8102 * t23488 - t10013 * t6088 / 4.0_f64 - t19990 * t3881 * t8103 / 8.0_f64 - 6.0_f64 * t6096 * t3881 * t1947 + 4.0_f64 * t1959 * t1211 * t8061 - t8074 * t10022 / 2.0_f64 - t3072 * t27474 - t8077 * t10022 / 4.0_f64 + 4.0_f64 * t1959 * t9999 * t616 + 2.0_f64 * t1959 * t3876 * t1947 - 24.0_f64 * t10043 * t8103 + 24.0_f64 * t19975 * t3881 * t1955 + 7.0_f64 / 2.0_f64 * t1976 * t10022 - 6.0_f64 * t6096 * t3876 * t1955 + 2.0_f64 * t72 * (t27530 + t27571);
    t27575
}
