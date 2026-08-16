//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1818/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1818(t4147: f64, t7535: f64, t36: f64, t68: f64, t1518: f64, t2051: f64, t2055: f64, t8107: f64, t1469: f64, t1450: f64, t211: f64, t9644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33183 = t4147 * t7535;
    let t33268 = t68 * t36;
    let t34251 = t2051 * t1518;
    let t34359 = t1518 * t2055;
    let t34495 = t4147 * t8107;
    let t34764 = t33268 * t1469;
    let t35927 = t8107 * t1450;
    let t39643 = 1.0_f64 / t9644 / t211;
    (t33183, t34251, t34359, t34495, t34764, t35927, t39643)
}
