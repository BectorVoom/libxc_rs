//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 785/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk785(t1518: f64, t7586: f64, t7888: f64, t7891: f64, t7893: f64, t8152: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t2127: f64, t2163: f64, t2165: f64, t508: f64, t569: f64, t651: f64, t7731: f64, t7734: f64, t7737: f64, t7744: f64, t7899: f64, t7903: f64, t7936: f64, t7938: f64, t8158: f64, t8233: f64) -> (f64, f64) {
    let t8237 = 2.0_f64 * t1518 * t7586 + t7888 + t7891 + t7893 + t8152;
    let t8240 = -t118 * t8233 - t1502 * t2163 - 2.0_f64 * t1519 * t7586 - t1843 * t2127 + t1911 * t2165 - t508 * t8152 + t569 * t8237 - 2.0_f64 * t651 * t8158 - t7731 - t7734 - t7737 - t7744 + t7899 + t7903 + t7936 - t7938;
    (t8237, t8240)
}
