//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 744/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk744(t495: f64, t6212: f64, t6211: f64, t6209: f64, t2182: f64, t489: f64, t548: f64, t2090: f64, t57: f64, t128: f64, t524: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6213 = t6212 * t495;
    let t6214 = t6211 * t6213;
    let t6215 = t6209 * t6214;
    let t6217 = t2182 * t489;
    let t6218 = t6217 * t548;
    let t6238 = t2090 * t57;
    let t6239 = t6238 * t128;
    let t6240 = t524 * t6239;
    let t6241 = t6240 * t540;
    (t6213, t6214, t6215, t6217, t6218, t6238, t6239, t6240, t6241)
}
