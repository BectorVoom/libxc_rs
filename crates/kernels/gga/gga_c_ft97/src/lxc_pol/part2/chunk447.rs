//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 447/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk447<F: Float>(t292: F, t2724: F, t287: F, t820: F, t2434: F, t2437: F, t2444: F, t2449: F, t2453: F, t817: F, t2689: F, t2691: F, t2693: F, t2720: F, t285: F, t800: F) -> (F, F, F, F, F) {
    let t293 = 0.1e-59 < t292;
    let t2725 = t287 * t2724;
    let t2726 = t820 * t820;
    let t2727 = t2725 * t2726;
    let t2730 = 0.11113000182098765433e-1 * t2434;
    let t2735 = -t2730 + 0.11113000182098765433e-1 * t2437 + 0.22226000364197530865e-1 * t2444 - 0.33339000546296296298e-1 * t2449 + 0.16669500273148148149e-1 * t2453;
    let t2736 = t817 * t2735;
    let t2739 = piecewise3(t293, -4.0 * t2691 * t2693 + 2.0 * t2720 * t800 + 2.0 * t2727 * t285 - t2736 * t285 + 2.0 * t2689, 0.0);
    (t2725, t2726, t2730, t2735, t2739)
}
