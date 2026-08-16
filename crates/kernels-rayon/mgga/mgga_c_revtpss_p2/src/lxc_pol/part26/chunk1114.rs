//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1114/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1114(t25207: f64, t51806: f64, t2257: f64, t890: f64, t10818: f64, t27159: f64, t2832: f64, t605: f64, t2408: f64, t2411: f64, t14365: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92765 = t25207 * t51806;
    let t92768 = t2257 * t890;
    let t92772 = t27159 * t10818;
    let t92779 = t605 * t2832;
    let t92783 = t605 * t2408;
    let t92790 = t2411 * t605;
    let t92791 = t92790 * t14365;
    let t92795 = t605 * t2430;
    (t92765, t92768, t92772, t92779, t92783, t92791, t92795)
}
