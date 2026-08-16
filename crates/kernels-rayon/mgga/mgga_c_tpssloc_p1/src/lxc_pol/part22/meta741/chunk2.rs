//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2445/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2445(t21370: f64, t2940: f64, t13847: f64, t17817: f64, t2986: f64, t21444: f64, t2987: f64, t13784: f64, t21122: f64, t21456: f64, t20217: f64, t2989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69471 = 0.51947577317044391277e2_f64 * t2940 * t21370;
    let t69487 = t2986 * t13847 * t17817;
    let t69496 = t2987 * t21444;
    let t69503 = t2986 * t13784 * t21122;
    let t69505 = t2987 * t21456;
    let t69515 = t2989 * t20217;
    (t69471, t69487, t69496, t69503, t69505, t69515)
}
