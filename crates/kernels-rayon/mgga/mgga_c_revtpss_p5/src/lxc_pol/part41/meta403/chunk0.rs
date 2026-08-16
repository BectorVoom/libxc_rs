//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1394/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1394(t1284: f64, t6564: f64, t6688: f64, t73: f64, t5458: f64, t1287: f64, t21257: f64, t1811: f64, t3766: f64, t460: f64, t3781: f64, t21040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21439 = t6564 * t1284;
    let t21442 = t6688 * t73;
    let t21443 = t21442 * t5458;
    let t21448 = t21257 * t1287;
    let t21451 = t3766 * t1811;
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21459 = t21040 * t5458;
    (t21439, t21443, t21448, t21452, t21456, t21459)
}
