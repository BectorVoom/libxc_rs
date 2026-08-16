//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1044/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1044(t66: f64, t7172: f64, t355: f64, t938: f64, t136363: f64, t22796: f64, t6441: f64, t136359: f64, t136485: f64, t136637: f64, t136642: f64, t136656: f64, t136930: f64, t145200: f64, t145205: f64, t145209: f64, t22603: f64, t22735: f64, t22736: f64, t25637: f64, t25654: f64, t3078: f64, t45500: f64, t53: f64, t5538: f64, t6445: f64, t7205: f64, t7318: f64, t929: f64) -> f64 {
    let t145247 = t7172 * t66;
    let t145255 = t355 * t938;
    let t145264 = t22796 * t136363 * t6441;
    let t145266 = 0.13359406463155864749e-8_f64 * t45500 * t22735 * t7318 * t929 + 0.30638775012461239605e-5_f64 * t22736 * t145200 - 0.1721820212247325051e-5_f64 * t5538 * t145205 - 0.25845121844514357744e-4_f64 * t22603 * t145209 - 4.0_f64 * t136485 * t25637 + 4.0_f64 * t145247 * t3078 - 0.15625977470667646633e-5_f64 * t136642 * t136637 * t6445 - 0.17816121467177433867e-2_f64 * t136930 * t25654 - 0.12690037786211307469e-3_f64 * t136656 * t7205 * t145255 * t53 + 0.31680880081247724282e-4_f64 * t22796 * t136359 * t6441 - 0.13200366700519885118e-5_f64 * t145264;
    t145266
}
