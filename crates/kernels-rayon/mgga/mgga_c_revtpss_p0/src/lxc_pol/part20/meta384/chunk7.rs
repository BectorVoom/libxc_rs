//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1409/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1409(t2408: f64, t2410: f64, t2832: f64, t775: f64, t10818: f64, t11071: f64, t198: f64, t207: f64, t2393: f64, t2403: f64, t2404: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t39857: f64, t39859: f64, t39861: f64, t40084: f64, t40088: f64, t40240: f64, t4541: f64) -> f64 {
    let t41151 = t2408 * t2408;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t41161 = t775 * t2832;
    let t41168 = -6.0_f64 * t198 * t207 * t41151 * t41154 + 72.0_f64 * t10818 * t2404 * t4541 - 36.0_f64 * t11071 * t2403 * t41161 + 18.0_f64 * t198 * t2393 * t40240 + t39799 + t39807 - t39813 - t39818 - t39823 + t39857 + t39859 - t39861 + t40084 + t40088;
    t41168
}
