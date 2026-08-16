//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2274/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2274(t15338: f64, t18542: f64, t3447: f64, t15293: f64, t19256: f64, t225: f64, t19211: f64, t3030: f64, t6150: f64, t3609: f64, t3623: f64, t18710: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t65139 = t3447 * t15338 * t18542;
    let t65142 = t3447 * t15338 * t15293;
    let t65203 = t19256 * t225;
    let t65208 = t19211 * t225;
    let t65253 = t6150 * t3030;
    let t65254 = t65253 * t3609;
    let t65262 = t65253 * t3623;
    let t65288 = t300 * t18710;
    (t65139, t65142, t65203, t65208, t65253, t65254, t65262, t65288)
}
