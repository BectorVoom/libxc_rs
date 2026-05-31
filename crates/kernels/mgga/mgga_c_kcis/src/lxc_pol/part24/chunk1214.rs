//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1214/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1214<F: Float>(t14665: F, t8081: F, t19843: F, t26871: F, t1203: F, t29081: F, t3330: F, t1820: F, t28071: F, t10491: F, t29042: F, t6735: F, t7766: F) -> (F, F, F, F, F, F) {
    let t99837 = F::cast_from(2.0_f64) * t14665 * t8081;
    let t99839 = F::cast_from(2.0_f64) * t26871 * t19843;
    let t99842 = F::cast_from(2.0_f64) * t3330 * t29081 * t1203;
    let t99845 = F::cast_from(4.0_f64) * t3330 * t28071 * t1820;
    let t99847 = F::cast_from(2.0_f64) * t10491 * t29042;
    let t99850 = F::cast_from(2.0_f64) * t3330 * t7766 * t6735;
    (t99837, t99839, t99842, t99845, t99847, t99850)
}
