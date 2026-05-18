//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 968/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk968<F: Float>(t49989: F, t701: F, t50062: F, t550: F, t14364: F, t296: F, t1: F, t787: F, t723: F, t2028: F, t2033: F, t2615: F, t326: F, t45287: F, t45288: F, t45298: F, t45300: F, t45304: F, t45308: F, t45315: F, t45319: F, t45323: F, t45326: F, t45329: F, t45331: F, t45335: F, t45343: F, t45349: F, t549: F, t6066: F, t7630: F, t825: F) -> (F, F, F, F, F) {
    let t50111 = t49989 * t701;
    let t50118 = t550 * t50062;
    let t50122 = t296 * t14364;
    let t50124 = t787 * t50122 * t1;
    let t50130 = t49989 * t723;
    let t50134 = -t45287 - t45288 - t45298 - t45300 - t45304 - t45308 + t45315 + F::new(0.92023022289409799224e1) * t2615 * t326 * t50111 - F::new(0.14300195980740170668e1) * t7630 * t6066 * t50111 + t45319 - t45323 + t45326 + F::new(0.39722766613167140743e-1) * t2033 * t549 * t50118 - F::new(0.39722766613167140743e-1) * t50124 * t2028 + F::new(0.12780975317973583225e0) * t45329 + F::new(0.9585731488480187419e0) * t45331 - F::new(0.21301625529955972042e0) * t45335 - F::new(0.18404604457881959845e2) * t825 * t326 * t50130 - t45343 + t45349;
    (t50111, t50118, t50122, t50130, t50134)
}
