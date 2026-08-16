//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 449/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk449(t2439: f64, t2456: f64, t1184: f64, t1206: f64, t1214: f64, t2136: f64, t2138: f64, t2140: f64, t2423: f64, t2426: f64, t2430: f64, t2432: f64, t2437: f64, t2441: f64, t2445: f64, t2449: f64, t2453: f64, t788: f64, t835: f64) -> f64 {
    let t2457 = t2439 * t2456;
    let t2460 = -0.56366309740899397906e-3_f64 * t835 * t2423 - 0.56366309740899397906e-3_f64 * t2426 * t788 - t1184 + t2136 - 0.33406432906439709826e-4_f64 * t2430 * t2432 - 0.2740028945738165176e-4_f64 * t2437 * t2441 - 0.33406432906439709826e-4_f64 * t2445 * t2449 - 0.2740028945738165176e-4_f64 * t2453 * t2457 - t2138 - t2140 + t1206 + t1214;
    t2460
}
