//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 700/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk700(t131: f64, t2570: f64, t205: f64, t242: f64, t2628: f64, t812: f64, t244: f64, t67: f64, t246: f64, t157: f64, t2658: f64, t228: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4177 = t2628 * t242;
    let t4178 = t812 * t4177;
    let t4179 = t244 * t67;
    let t4180 = t4179 * t246;
    let t4194 = t2658 * t157;
    let t4225 = t228 * t68;
    (t4127, t4178, t4179, t4180, t4194, t4225)
}
