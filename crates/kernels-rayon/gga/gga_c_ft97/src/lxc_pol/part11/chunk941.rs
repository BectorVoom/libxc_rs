//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 941/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk941(t12137: f64, t1643: f64, t2258: f64, t2259: f64, t2265: f64, t2266: f64, t2282: f64, t2294: f64, t358: f64, t3613: f64, t3621: f64, t363: f64, t37264: f64, t37269: f64, t37311: f64, t37362: f64, t39483: f64, t39485: f64, t39487: f64, t39495: f64, t39514: f64, t39524: f64, t39550: f64, t39568: f64, t631: f64, t637: f64, t639: f64, t643: f64, t7955: f64, t7959: f64, t7973: f64, t8618: f64, t8619: f64, t8654: f64) -> f64 {
    let t39574 = 8.0_f64 / 9.0_f64 * t39483 - 8.0_f64 * t39485 - 16.0_f64 / 27.0_f64 * t2265 * t39487 * t7955 * t643 + 4.0_f64 / 9.0_f64 * t2265 * t12137 * t37311 + 2.0_f64 * t2265 * t39495 * t1643 * t2282 + 8.0_f64 / 3.0_f64 * t2265 * t8654 * t7959 * t643 - 4.0_f64 / 3.0_f64 * t2265 * t2266 * t7973 * t643 - 4.0_f64 / 3.0_f64 * t2265 * t3621 * t37264 + 2.0_f64 / 9.0_f64 * t2265 * t3613 * t37269 - 16.0_f64 * t2265 * t39514 * t8619 * t358 * t363 + t631 * t2258 * t2259 * t37362 / 6.0_f64 - 10.0_f64 * t39524 + 36.0_f64 * t631 * t637 * t8618 * t2282 * t2294 + t631 * t637 * t639 * (t39550 + t39568) / 2.0_f64;
    t39574
}
