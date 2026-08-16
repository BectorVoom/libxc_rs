//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1221/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1221(t26359: f64, t9303: f64, t10146: f64, t2097: f64, t25921: f64, t25930: f64, t26304: f64, t26371: f64, t7295: f64, t7296: f64, t94721: f64, t94868: f64, t96556: f64, t96559: f64, t96561: f64, t96564: f64, t96565: f64, t96567: f64, t96570: f64, t96577: f64, t96584: f64, t96588: f64) -> f64 {
    let t96591 = 0.26019841438354088051e-2_f64 * t9303 * t26359;
    let t96594 = 0.16463622957338778996e-1_f64 * t96556 + 0.19514881078765566037e-2_f64 * t96559 - 0.39029762157531132076e-1_f64 * t96561 - t96564 + 0.57824187921367996415e-1_f64 * t96565 + 0.38554277296572111609e-1_f64 * t96567 + 0.32927245914677557992e-1_f64 * t96570 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t94721 - 0.58544643236296698113e-1_f64 * t96577 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2097 * t10146 - t96584 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t94868 + 0.77108554593144223218e-1_f64 * t96588 + t96591 - 0.78062653693846795158e1_f64 * t25921 * t26371;
    t96594
}
