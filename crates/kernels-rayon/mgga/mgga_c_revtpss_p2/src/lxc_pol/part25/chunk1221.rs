//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1221/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1221(t25207: f64, t51775: f64, t41161: f64, t27383: f64, t51792: f64, t51806: f64, t2257: f64, t890: f64, t10818: f64, t27159: f64, t2411: f64, t25435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92753 = t25207 * t51775;
    let t92759 = t25207 * t41161;
    let t92762 = t27383 * t51792;
    let t92765 = t25207 * t51806;
    let t92768 = t2257 * t890;
    let t92772 = t27159 * t10818;
    let t92775 = t25435 * t2411;
    (t92753, t92759, t92762, t92765, t92768, t92772, t92775)
}
