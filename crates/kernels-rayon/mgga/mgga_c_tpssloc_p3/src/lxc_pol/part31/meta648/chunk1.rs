//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1923/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1923(t16817: f64, t1888: f64, t82018: f64, t16825: f64, t22996: f64, t23168: f64, t28346: f64, t28338: f64, t81591: f64, t252: f64, t5544: f64, t22986: f64, t6646: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98402 = t1888 * t82018 * t16817;
    let t98405 = t1888 * t22996 * t16825;
    let t98416 = t23168 * t28346;
    let t98420 = t81591 * t28338;
    let t98422 = t252 * t5544;
    let t98425 = t22986 * t6646 * t98422 * t829;
    (t98402, t98405, t98416, t98420, t98422, t98425)
}
