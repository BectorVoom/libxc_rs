//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1389/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1389<F: Float>(t102867: F, t102869: F, t102871: F, t102873: F, t102875: F, t102877: F, t102879: F, t102881: F, t102883: F, t102886: F, t102914: F, t102916: F, t102919: F, t102922: F, t102924: F, t102926: F, t102928: F, t102930: F, t102932: F, t102934: F, t102963: F, t102965: F, t102967: F, t102969: F, t102971: F, t102973: F, t102975: F, t102978: F, t102980: F, t102982: F, t103010: F, t103012: F, t103014: F, t103016: F, t103018: F, t103020: F, t103022: F, t103024: F, t103026: F, t103028: F, t103821: F, t103845: F, t103870: F, t103894: F, t1506: F) -> F {
    let t103898 = t1506 * (-t103014 / F::new(48.0) + t102916 / F::new(4.0) - t102978 / F::new(144.0) - F::new(2.0) / F::new(9.0) * t103016 + t103024 / F::new(72.0) + t102873 / F::new(9.0) + t102875 / F::new(64.0) + t102919 / F::new(24.0) + t103894 + t102881 / F::new(96.0) - t103010 / F::new(12.0) + t103012 / F::new(24.0) + t102980 / F::new(96.0) + F::new(3.0) / F::new(64.0) * t102883 + t102886 / F::new(27.0) - F::new(19.0) / F::new(72.0) * t102914 - t102963 / F::new(96.0) - t103026 / F::new(16.0) + t103028 / F::new(12.0) + t102965 / F::new(8.0) + t102867 / F::new(48.0) + t102869 / F::new(36.0) + t102932 / F::new(12.0) + t103845 - t102982 / F::new(32.0) + t102926 / F::new(128.0) + t103870 + t102967 / F::new(128.0) - t102969 / F::new(72.0) + t102922 / F::new(24.0) - t102924 / F::new(12.0) - t103018 / F::new(9.0) - t103020 / F::new(128.0) + t102973 / F::new(12.0) - t102975 / F::new(288.0) - t103022 / F::new(96.0) - t102871 / F::new(12.0) + t103821 - F::new(3.0) / F::new(8.0) * t102971 - t102934 / F::new(288.0) + t102928 / F::new(54.0) - t102930 / F::new(24.0) + t102877 / F::new(9.0) - F::new(19.0) / F::new(54.0) * t102879);
    t103898
}
