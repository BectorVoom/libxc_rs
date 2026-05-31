//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1188/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1188<F: Float>(t19847: F, t19850: F, t19852: F, t19854: F, t19858: F, t19860: F, t19863: F, t19866: F, t19868: F, t19871: F, t19873: F, t19875: F, t19877: F, t19880: F, t19883: F, t19886: F, t19888: F, t19892: F) -> F {
    let t19894 = -t19847 / F::cast_from(288.0_f64) + t19850 / F::cast_from(96.0_f64) + t19852 / F::cast_from(48.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19854 - t19858 / F::cast_from(48.0_f64) - t19860 / F::cast_from(12.0_f64) + t19863 / F::cast_from(36.0_f64) - t19866 / F::cast_from(128.0_f64) + t19868 / F::cast_from(24.0_f64) - t19871 / F::cast_from(24.0_f64) - t19873 / F::cast_from(12.0_f64) + t19875 / F::cast_from(3.0_f64) + t19877 / F::cast_from(96.0_f64) - t19880 / F::cast_from(72.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t19883 + t19886 / F::cast_from(24.0_f64) - t19888 / F::cast_from(6.0_f64) + t19892 / F::cast_from(36.0_f64);
    t19894
}
