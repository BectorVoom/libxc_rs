//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1839/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1839(t1343: f64, t1450: f64, t1868: f64, t198: f64, t21937: f64, t39419: f64, t39422: f64, t4139: f64, t46292: f64, t46297: f64, t46303: f64, t532: f64, t5536: f64, t6816: f64, t6836: f64, t86731: f64, t86839: f64, t91826: f64, t91952: f64, t91953: f64, t91954: f64, t91955: f64, t92229: f64, t92248: f64, t92267: f64, t92434: f64) -> f64 {
    let t92446 = 36.0_f64 * t198 * t86839 * t6816 + t46292 - t46297 + 3.0_f64 * t198 * t1343 * t91826 + t198 * t532 * (t92229 + t92248 + t92267 + t92434) * t1450 - t39419 - t39422 + t46303 + t91952 - t91953 + t91954 + t91955 + 36.0_f64 * t5536 * t21937 * t6836 + 12.0_f64 * t4139 * t86731 * t1868;
    t92446
}
