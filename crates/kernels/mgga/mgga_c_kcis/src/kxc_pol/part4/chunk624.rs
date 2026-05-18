//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 624/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk624<F: Float>(t2815: F, t3338: F, t3337: F, t1130: F, t3190: F, t376: F, t375: F, t359: F, t3219: F, t387: F, t382: F, t280: F, t383: F, t980: F) -> (F, F, F, F, F, F, F, F) {
    let t3339 = t3338 * t2815;
    let t3340 = t3337 * t3339;
    let t3342 = t1130 * t3190;
    let t3343 = t376 * t3342;
    let t3344 = t375 * t3343;
    let t3346 = F::new(1.0) / t359;
    let t3347 = t3346 * t3219;
    let t3348 = t387 * t3347;
    let t3349 = t382 * t3348;
    let t3353 = F::new(1.0) / t280 / t383 / t980;
    (t3339, t3340, t3343, t3344, t3346, t3348, t3349, t3353)
}
