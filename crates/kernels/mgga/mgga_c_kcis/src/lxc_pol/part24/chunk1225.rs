//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1225/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1225<F: Float>(t28059: F, t5062: F, t19937: F, t7754: F, t19849: F, t26930: F, t19655: F, t3338: F, t26938: F, t29062: F, t26924: F, t99945: F, t99948: F, t99950: F, t99952: F, t99954: F, t99956: F, t99958: F, t99960: F, t99962: F, t99964: F, t99966: F, t99968: F) -> (F, F, F, F, F, F, F) {
    let t99970 = t28059 * t5062;
    let t99972 = t7754 * t19937;
    let t99974 = t26930 * t19849;
    let t99977 = t7754 * t3338 * t19655;
    let t99979 = t26938 * t29062;
    let t99981 = t26924 * t29062;
    let t99983 = -t99945 / F::new(48.0) - t99948 / F::new(144.0) - t99950 / F::new(9.0) + t99952 / F::new(128.0) - t99954 / F::new(12.0) - t99956 / F::new(12.0) - t99958 / F::new(12.0) - t99960 / F::new(24.0) + t99962 / F::new(54.0) - t99964 / F::new(24.0) - t99966 / F::new(16.0) - t99968 / F::new(3.0) - t99970 / F::new(12.0) + t99972 / F::new(54.0) + t99974 / F::new(48.0) + t99977 / F::new(24.0) + t99979 / F::new(24.0) - t99981 / F::new(9.0);
    (t99970, t99972, t99974, t99977, t99979, t99981, t99983)
}
