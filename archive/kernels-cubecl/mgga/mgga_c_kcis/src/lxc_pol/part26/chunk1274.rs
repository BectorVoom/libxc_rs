//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1274/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1274<F: Float>(t17308: F, t8207: F, t28570: F, t48058: F, t22714: F, t7940: F, t27491: F, t7397: F, t28778: F, t28853: F, t28713: F, t6140: F) -> (F, F, F, F, F, F) {
    let t101837 = F::cast_from(2.0_f64) * t17308 * t8207;
    let t101839 = F::cast_from(12.0_f64) * t48058 * t28570;
    let t101840 = t7940 * t22714;
    let t101841 = t27491 * t7397;
    let t101849 = t28853 * t28778;
    let t101853 = t28713 * t6140;
    (t101837, t101839, t101840, t101841, t101849, t101853)
}
