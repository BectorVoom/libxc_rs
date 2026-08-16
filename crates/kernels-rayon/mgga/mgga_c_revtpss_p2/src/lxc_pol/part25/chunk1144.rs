//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1144/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1144(t25460: f64, t7150: f64, t11120: f64, t359: f64, t1976: f64, t3270: f64, t1096: f64, t7135: f64, t7160: f64, t1982: f64, t994: f64, t3325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25461 = t7150 * t25460;
    let t25464 = t11120 * t359;
    let t25465 = t1976 * t3270;
    let t25466 = t25464 * t25465;
    let t25470 = t7160 * t7135 * t1096;
    let t25473 = t1982 * t25460;
    let t25476 = t994 * t25460;
    let t25479 = t1976 * t3325;
    (t25461, t25464, t25465, t25466, t25470, t25473, t25476, t25479)
}
