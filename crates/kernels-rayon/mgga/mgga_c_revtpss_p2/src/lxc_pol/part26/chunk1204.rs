//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1204/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1204(t26230: f64, t9681: f64, t94674: f64, t7523: f64, t94610: f64, t96232: f64, t96234: f64, t96237: f64, t96240: f64, t96243: f64, t96246: f64, t96249: f64, t96253: f64, t96257: f64, t96260: f64, t96262: f64, t96265: f64, t96269: f64) -> (f64, f64) {
    let t96271 = t26230 * t9681;
    let t96272 = t94674 * t96271;
    let t96274 = 0.21684070470512998656e-1_f64 * t96232 + 0.77108554593144223218e-1_f64 * t96234 - 0.15421710918628844643e0_f64 * t96237 + 0.15421710918628844643e0_f64 * t96240 - 0.43368140941025997312e-1_f64 * t96243 - 0.51405703062096148812e-1_f64 * t96246 + 0.38554277296572111609e-1_f64 * t96249 - 0.19514881078765566038e-2_f64 * t96253 - t96257 - 0.68549505033305214441e-2_f64 * t96260 - 0.38554277296572111609e-1_f64 * t96262 - 0.10281140612419229762e0_f64 * t96265 + 0.26020884564615598386e1_f64 * t94610 * t7523 - 0.21684070470512998656e-1_f64 * t96269 + 0.13010442282307799194e0_f64 * t96272;
    (t96271, t96274)
}
