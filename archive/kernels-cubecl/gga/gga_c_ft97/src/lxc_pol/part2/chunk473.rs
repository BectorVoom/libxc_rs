//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 473/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk473<F: Float>(t2719: F, t291: F, t289: F, t815: F, t287: F, t820: F, t2434: F, t2437: F, t2444: F, t2449: F, t2453: F, t817: F) -> (F, F, F, F, F, F, F, F) {
    let t2720 = t291 * t2719;
    let t2724 = F::cast_from(1.0_f64) / t815 / t289;
    let t2725 = t287 * t2724;
    let t2726 = t820 * t820;
    let t2727 = t2725 * t2726;
    let t2730 = F::cast_from(0.11113000182098765433e-1_f64) * t2434;
    let t2735 = -t2730 + F::cast_from(0.11113000182098765433e-1_f64) * t2437 + F::cast_from(0.22226000364197530865e-1_f64) * t2444 - F::cast_from(0.33339000546296296298e-1_f64) * t2449 + F::cast_from(0.16669500273148148149e-1_f64) * t2453;
    let t2736 = t817 * t2735;
    (t2720, t2724, t2725, t2726, t2727, t2730, t2735, t2736)
}
