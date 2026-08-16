//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 968/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk968(t49989: f64, t701: f64, t50062: f64, t550: f64, t14364: f64, t296: f64, t1: f64, t787: f64, t723: f64, t2028: f64, t2033: f64, t2615: f64, t326: f64, t45287: f64, t45288: f64, t45298: f64, t45300: f64, t45304: f64, t45308: f64, t45315: f64, t45319: f64, t45323: f64, t45326: f64, t45329: f64, t45331: f64, t45335: f64, t45343: f64, t45349: f64, t549: f64, t6066: f64, t7630: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t50111 = t49989 * t701;
    let t50118 = t550 * t50062;
    let t50122 = t296 * t14364;
    let t50124 = t787 * t50122 * t1;
    let t50130 = t49989 * t723;
    let t50134 = -t45287 - t45288 - t45298 - t45300 - t45304 - t45308 + t45315 + 0.92023022289409799224e1_f64 * t2615 * t326 * t50111 - 0.14300195980740170668e1_f64 * t7630 * t6066 * t50111 + t45319 - t45323 + t45326 + 0.39722766613167140743e-1_f64 * t2033 * t549 * t50118 - 0.39722766613167140743e-1_f64 * t50124 * t2028 + 0.12780975317973583225e0_f64 * t45329 + 0.9585731488480187419e0_f64 * t45331 - 0.21301625529955972042e0_f64 * t45335 - 0.18404604457881959845e2_f64 * t825 * t326 * t50130 - t45343 + t45349;
    (t50111, t50118, t50122, t50130, t50134)
}
