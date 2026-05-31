//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 615/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk615<F: Float>(t245: F, t1178: F, t21: F, t267: F, t4431: F, t5: F, t5186: F, t920: F, t2639: F, t992: F, t1212: F, t231: F, t1218: F, t1526: F, t2320: F, t2638: F, t342: F, t343: F) -> (F, F, F, F) {
    let t246 = F::cast_from(10000000.0_f64) <= t245;
    let t5197 = piecewise3::<F>(t246, F::cast_from(0.0_f64), t5 * t5186 * t21 / F::cast_from(4.0_f64) + t5 * t1178 * t920 / F::cast_from(2.0_f64) + t5 * t267 * t4431 / F::cast_from(4.0_f64));
    let t5198 = t2639 * t992;
    let t5202 = t231 * t1212;
    let t5206 = t1218 - t2638 - t1526 * t2320 * t5198 / F::cast_from(12.0_f64) - t342 * t343 * t5202 / F::cast_from(4.0_f64);
    (t5197, t5198, t5202, t5206)
}
