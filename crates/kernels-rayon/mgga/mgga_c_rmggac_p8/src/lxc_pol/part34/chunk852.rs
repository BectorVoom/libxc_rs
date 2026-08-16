//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 852/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk852(t27: f64, t9157: f64, t16069: f64, t69609: f64, t9163: f64, t16074: f64, t68760: f64, t8450: f64, t14167: f64, t15291: f64, t34828: f64, t45468: f64) -> (f64, f64, f64, f64, f64) {
    let t75260 = t27 * t9157;
    let t75262 = t69609 * t16069 * t75260;
    let t75264 = t27 * t9163;
    let t75266 = t69609 * t16074 * t75264;
    let t75268 = t8450 * t68760;
    let t75269 = t75268 * t14167;
    let t75271 = t34828 * t15291;
    let t75273 = t45468 * t15291;
    (t75262, t75266, t75269, t75271, t75273)
}
