//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1117/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1117(t121239: f64, t32268: f64, t121133: f64, t25900: f64, t1385: f64, t240: f64, t27: f64, t119967: f64, t121204: f64, t13847: f64, t1399: f64, t121086: f64, t32710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t121240 = t32268 * t121239;
    let t121241 = t121133 * t25900;
    let t121242 = t121240 * t121241;
    let t121245 = t1385 * t27 * t240;
    let t121246 = t119967 * t121245;
    let t121248 = t13847 * t121204 * t1399;
    let t121249 = t121246 * t121248;
    let t121251 = t32710 * t121086;
    (t121240, t121241, t121242, t121245, t121246, t121248, t121249, t121251)
}
