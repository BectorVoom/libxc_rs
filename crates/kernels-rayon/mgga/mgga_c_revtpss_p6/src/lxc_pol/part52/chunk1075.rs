//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1075/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1075(t31755: f64, t33674: f64, t1544: f64, t2747: f64, t31756: f64, t31767: f64, t1579: f64, t31772: f64, t4364: f64, t1568: f64, t8477: f64, t8485: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33675 = t31755 * t33674;
    let t33678 = t2747 * t31756 * t1544;
    let t33679 = t31767 * t33678;
    let t33682 = t4364 * t31772 * t1579;
    let t33683 = t31767 * t33682;
    let t33695 = t8477 * t1568;
    let t33711 = t33695 * t8485;
    (t33675, t33678, t33679, t33682, t33683, t33695, t33711)
}
