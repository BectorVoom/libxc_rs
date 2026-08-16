//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2154/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2154(t11018: f64, t225: f64, t11016: f64, t11064: f64, t42332: f64, t11058: f64, t3185: f64, t42741: f64, t1014: f64, t42340: f64, t42341: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43431 = t11018 * t225;
    let t43440 = t11016 * t225;
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    let t43480 = t42741 * t3185;
    let t43503 = t42340 * t42341 * t1014;
    let t43515 = t42340 * t42341 * t3127;
    (t43431, t43440, t43470, t43473, t43480, t43503, t43515)
}
