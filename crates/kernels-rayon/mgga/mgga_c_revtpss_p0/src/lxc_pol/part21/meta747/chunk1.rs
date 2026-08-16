//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2622/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622(t48269: f64, t47019: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t47003: f64, t48258: f64, t48259: f64, t48261: f64, t48263: f64, t48264: f64, t48265: f64, t48266: f64, t48268: f64) -> (f64, f64, f64) {
    let t48270 = 0.51947577317044391277e2_f64 * t48269;
    let t48271 = 960.0_f64 * t47019;
    let t48272 = t47003 - t48258 + t48259 + t39773 + t48261 - t48263 - t39783 - t39786 - t39791 - t39795 + t48264 - t48265 - t48266 + t48268 - t48270 - t48271 + t39799;
    (t48270, t48271, t48272)
}
