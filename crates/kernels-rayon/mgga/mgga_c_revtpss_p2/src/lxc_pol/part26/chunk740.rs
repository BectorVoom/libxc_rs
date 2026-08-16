//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 740/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk740(t1331: f64, t3857: f64, t189: f64, t9363: f64, t512: f64, t3850: f64, t72: f64, t757: f64, t2619: f64, t3825: f64, t1333: f64, t3863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9559 = t3857 * t1331;
    let t9560 = 60.0_f64 * t9559;
    let t9561 = t9363 * t189;
    let t9562 = t512 * t9561;
    let t9563 = t3850 * t72;
    let t9564 = t9563 * t757;
    let t9565 = 0.54934341918019635162e-3_f64 * t9564;
    let t9566 = t3825 * t2619;
    let t9567 = 0.73245789224026180216e-3_f64 * t9566;
    let t9569 = 60.0_f64 * t3857 * t1333;
    let t9570 = t3863 * t1331;
    (t9560, t9562, t9565, t9567, t9569, t9570)
}
