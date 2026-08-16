//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1389/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1389<F: Float>(t102867: F, t102869: F, t102871: F, t102873: F, t102875: F, t102877: F, t102879: F, t102881: F, t102883: F, t102886: F, t102914: F, t102916: F, t102919: F, t102922: F, t102924: F, t102926: F, t102928: F, t102930: F, t102932: F, t102934: F, t102963: F, t102965: F, t102967: F, t102969: F, t102971: F, t102973: F, t102975: F, t102978: F, t102980: F, t102982: F, t103010: F, t103012: F, t103014: F, t103016: F, t103018: F, t103020: F, t103022: F, t103024: F, t103026: F, t103028: F, t103821: F, t103845: F, t103870: F, t103894: F, t1506: F) -> F {
    let t103898 = t1506 * (-t103014 / F::cast_from(48.0_f64) + t102916 / F::cast_from(4.0_f64) - t102978 / F::cast_from(144.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t103016 + t103024 / F::cast_from(72.0_f64) + t102873 / F::cast_from(9.0_f64) + t102875 / F::cast_from(64.0_f64) + t102919 / F::cast_from(24.0_f64) + t103894 + t102881 / F::cast_from(96.0_f64) - t103010 / F::cast_from(12.0_f64) + t103012 / F::cast_from(24.0_f64) + t102980 / F::cast_from(96.0_f64) + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t102883 + t102886 / F::cast_from(27.0_f64) - F::cast_from(19.0_f64) / F::cast_from(72.0_f64) * t102914 - t102963 / F::cast_from(96.0_f64) - t103026 / F::cast_from(16.0_f64) + t103028 / F::cast_from(12.0_f64) + t102965 / F::cast_from(8.0_f64) + t102867 / F::cast_from(48.0_f64) + t102869 / F::cast_from(36.0_f64) + t102932 / F::cast_from(12.0_f64) + t103845 - t102982 / F::cast_from(32.0_f64) + t102926 / F::cast_from(128.0_f64) + t103870 + t102967 / F::cast_from(128.0_f64) - t102969 / F::cast_from(72.0_f64) + t102922 / F::cast_from(24.0_f64) - t102924 / F::cast_from(12.0_f64) - t103018 / F::cast_from(9.0_f64) - t103020 / F::cast_from(128.0_f64) + t102973 / F::cast_from(12.0_f64) - t102975 / F::cast_from(288.0_f64) - t103022 / F::cast_from(96.0_f64) - t102871 / F::cast_from(12.0_f64) + t103821 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t102971 - t102934 / F::cast_from(288.0_f64) + t102928 / F::cast_from(54.0_f64) - t102930 / F::cast_from(24.0_f64) + t102877 / F::cast_from(9.0_f64) - F::cast_from(19.0_f64) / F::cast_from(54.0_f64) * t102879);
    t103898
}
