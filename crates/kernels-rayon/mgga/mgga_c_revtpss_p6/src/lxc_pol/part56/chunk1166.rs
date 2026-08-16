//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1166/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1166(t2042: f64, t29480: f64, t2170: f64, t28268: f64, t28277: f64, t28265: f64, t28280: f64, t35018: f64, t575: f64, t125211: f64, t125213: f64, t125215: f64, t125217: f64, t129251: f64, t129253: f64, t129255: f64, t129257: f64, t129273: f64, t129277: f64, t129279: f64, t1518: f64, t2322: f64, t33346: f64, t33550: f64, t33578: f64, t33580: f64, t33583: f64, t34882: f64, t4254: f64, t4297: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129564 = t29480 * t2042;
    let t129570 = t2170 * t28268;
    let t129572 = t2170 * t28277;
    let t129574 = t2170 * t28265;
    let t129577 = t2170 * t28280;
    let t131183 = t35018 * t575;
    let t131200 = -2.0_f64 * t1518 * t33550 * t651 - 2.0_f64 * t2322 * t34882 - 2.0_f64 * t33346 * t4297 - 2.0_f64 * t34882 * t4254 - t125211 - t125213 - t125215 - t125217 - 4.0_f64 * t129251 - 4.0_f64 * t129253 - 4.0_f64 * t129255 - 4.0_f64 * t129257 - 4.0_f64 * t129273 - 4.0_f64 * t129277 - 4.0_f64 * t129279 - t33578 - t33580 - t33583;
    (t129564, t129570, t129572, t129574, t129577, t131183, t131200)
}
