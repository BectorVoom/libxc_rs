//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1826/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1826(t215: f64, t6916: f64, t225: f64, t3787: f64, t562: f64, t22751: f64, t26385: f64, t81149: f64, t81187: f64, t81197: f64, t26389: f64, t26467: f64, t6914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91004 = t6916 * t215;
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91010 = t22751 * t26385;
    let t91018 = 0.16449340668482264365e-1_f64 * t81149;
    let t91043 = 0.25587863262083522346e0_f64 * t81187;
    let t91045 = 0.3289868133696452873e-1_f64 * t81197;
    let t91064 = t22751 * t26389;
    let t91076 = t6914 * t26467;
    (t91004, t91005, t91006, t91010, t91018, t91043, t91045, t91064, t91076)
}
