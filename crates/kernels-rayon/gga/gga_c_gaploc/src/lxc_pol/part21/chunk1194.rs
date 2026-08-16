//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1194/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1194(t3444: f64, t7211: f64, t10743: f64, t2549: f64, t24908: f64, t2562: f64, t883: f64, t943: f64, t10683: f64, t7137: f64, t10682: f64, t2042: f64, t2508: f64) -> (f64, f64, f64, f64, f64) {
    let t32116 = t7211 * t3444;
    let t32117 = 0.32043859292259267849e-3_f64 * t32116;
    let t32118 = t2549 * t10743;
    let t32119 = 0.64087718584518535698e-3_f64 * t32118;
    let t32122 = t943 * t2562 * t883 * t24908;
    let t32123 = 0.32043859292259267849e-3_f64 * t32122;
    let t32125 = 0.20508069947045931424e-1_f64 * t7137 * t10683;
    let t32128 = 0.76905262301422242837e-2_f64 * t2508 * t2042 * t10682;
    (t32117, t32119, t32123, t32125, t32128)
}
