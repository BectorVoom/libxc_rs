//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 559/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk559<F: Float>(t3339: F, t3498: F, t980: F, t161: F, t3230: F, t3233: F, t3397: F, t3409: F, t3332: F, t3330: F, t3444: F, t3453: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3601 = F::new(4.0) / F::new(9.0) * t3339;
    let t3613 = F::cast_from(6.5831116232644735_f64) * t980 * t3498;
    let t3614 = t161 * t3230;
    let t3616 = t161 * t3233;
    let t3629 = F::cast_from(2.1389894610184537_f64) * t3397;
    let t3632 = F::cast_from(9.625452574583042_f64) * t3409;
    let t3633 = F::cast_from(0.8533333333333334_f64) * t3332;
    let t3634 = F::cast_from(0.14222222222222222_f64) * t3339;
    let t3643 = F::new(0.64) * t3330;
    let t3650 = F::cast_from(9.625452574583042_f64) * t3444;
    let t3652 = F::cast_from(25.667873532221446_f64) * t3453;
    (t3601, t3613, t3614, t3616, t3629, t3632, t3633, t3634, t3643, t3650, t3652)
}
