//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 412/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk412<F: Float>(t147: F, t184: F, t2299: F, t21: F, t648: F, t363: F, t649: F, t1580: F, t185: F, t2236: F, t2240: F, t5: F, t620: F, t623: F, t650: F) -> (F, F, F, F, F, F, F) {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t2300 = t2299 * t184;
    let t2301 = t2300 * t21;
    let t2304 = t648 * t648;
    let t2305 = t2304 * t184;
    let t2306 = t2305 * t21;
    let t2309 = t649 * t363;
    let t2316 = piecewise3::<F>(t148, F::new(0.0), t5 * t2236 * t21 / F::new(4.0) + t2240 * t650 / F::new(2.0) + t5 * t620 * t363 / F::new(2.0) + t623 * t2301 / F::new(4.0) + t623 * t2306 / F::new(4.0) + t623 * t2309 / F::new(2.0) + t5 * t185 * t1580 / F::new(4.0));
    (t2300, t2301, t2304, t2305, t2306, t2309, t2316)
}
