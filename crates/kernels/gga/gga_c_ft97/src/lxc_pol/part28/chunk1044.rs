//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1044/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1044<F: Float>(t66: F, t7172: F, t355: F, t938: F, t136363: F, t22796: F, t6441: F, t136359: F, t136485: F, t136637: F, t136642: F, t136656: F, t136930: F, t145200: F, t145205: F, t145209: F, t22603: F, t22735: F, t22736: F, t25637: F, t25654: F, t3078: F, t45500: F, t53: F, t5538: F, t6445: F, t7205: F, t7318: F, t929: F) -> F {
    let t145247 = t7172 * t66;
    let t145255 = t355 * t938;
    let t145264 = t22796 * t136363 * t6441;
    let t145266 = F::cast_from(0.13359406463155864749e-8_f64) * t45500 * t22735 * t7318 * t929 + F::cast_from(0.30638775012461239605e-5_f64) * t22736 * t145200 - F::cast_from(0.1721820212247325051e-5_f64) * t5538 * t145205 - F::cast_from(0.25845121844514357744e-4_f64) * t22603 * t145209 - F::new(4.0) * t136485 * t25637 + F::new(4.0) * t145247 * t3078 - F::cast_from(0.15625977470667646633e-5_f64) * t136642 * t136637 * t6445 - F::cast_from(0.17816121467177433867e-2_f64) * t136930 * t25654 - F::cast_from(0.12690037786211307469e-3_f64) * t136656 * t7205 * t145255 * t53 + F::cast_from(0.31680880081247724282e-4_f64) * t22796 * t136359 * t6441 - F::cast_from(0.13200366700519885118e-5_f64) * t145264;
    t145266
}
