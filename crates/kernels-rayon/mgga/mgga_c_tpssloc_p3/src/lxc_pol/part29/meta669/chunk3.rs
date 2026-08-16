//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2238/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2238(t16311: f64, t3788: f64, t3791: f64, t6936: f64, t80784: f64, t80792: f64, t80794: f64, t1339: f64, t1825: f64, t26288: f64, t3734: f64, t80780: f64, t80789: f64, t80796: f64, t80801: f64, t80807: f64, t80814: f64, t80821: f64, t80826: f64, t80828: f64, t91226: f64, t91229: f64, t91233: f64, t91237: f64) -> f64 {
    let t91241 = t6936 * t3788 * t16311 * t3791;
    let t91244 = 0.33643963411783659044e-4_f64 * t80784;
    let t91246 = 0.10541775202358879834e-2_f64 * t80792;
    let t91247 = 119.0_f64 / 3456.0_f64 * t80794;
    let t91256 = t26288 * t1339 * t1825 * t3734;
    let t91258 = t91226 - 0.20186378047070195427e-3_f64 * t91229 - 0.24223653656484234512e-2_f64 * t91233 - 0.12111826828242117256e-2_f64 * t91237 + 0.12111826828242117256e-2_f64 * t91241 - 0.63250651214153279005e-2_f64 * t80780 + t91244 + 0.33643963411783659045e-4_f64 * t80789 - t91246 + t91247 - 7.0_f64 / 2304.0_f64 * t80796 - 0.6728792682356731809e-4_f64 * t80801 + 0.33643963411783659045e-4_f64 * t80807 + 0.20186378047070195427e-3_f64 * t80814 - 7.0_f64 / 288.0_f64 * t80821 - t80826 - 7.0_f64 / 48.0_f64 * t80828 - 0.84782787797694820792e-2_f64 * t91256;
    t91258
}
