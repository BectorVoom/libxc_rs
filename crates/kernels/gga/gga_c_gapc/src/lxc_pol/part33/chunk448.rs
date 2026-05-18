//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 448/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk448<F: Float>(t2439: F, t2456: F, t1184: F, t1206: F, t1214: F, t2136: F, t2138: F, t2140: F, t2423: F, t2426: F, t2430: F, t2432: F, t2437: F, t2441: F, t2445: F, t2449: F, t2453: F, t788: F, t835: F) -> F {
    let t2457 = t2439 * t2456;
    let t2460 = -F::new(0.56366309740899397906e-3) * t835 * t2423 - F::new(0.56366309740899397906e-3) * t2426 * t788 - t1184 + t2136 - F::new(0.33406432906439709826e-4) * t2430 * t2432 - F::new(0.2740028945738165176e-4) * t2437 * t2441 - F::new(0.33406432906439709826e-4) * t2445 * t2449 - F::new(0.2740028945738165176e-4) * t2453 * t2457 - t2138 - t2140 + t1206 + t1214;
    t2460
}
