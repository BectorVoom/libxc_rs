//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1313/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1313<F: Float>(t99945: F, t99948: F, t99950: F, t99952: F, t99954: F, t99956: F, t99958: F, t99960: F, t99962: F, t99964: F, t99966: F, t99968: F, t99970: F, t99972: F, t99974: F, t99977: F, t99979: F, t99981: F) -> F {
    let t101681 = -F::new(0.53958333333333333333e-1) * t99945 - F::new(0.17986111111111111111e-1) * t99948 - F::new(0.28777777777777777778e0) * t99950 + F::new(0.20234375e-1) * t99952 - F::new(0.125e0) * t99954 - F::new(0.125e0) * t99956 - F::new(0.21583333333333333333e0) * t99958 - F::new(0.10791666666666666667e0) * t99960 + F::new(0.47962962962962962964e-1) * t99962 - F::new(0.10791666666666666667e0) * t99964 - F::new(0.9375e-1) * t99966 - F::new(0.5e0) * t99968 - F::new(0.125e0) * t99970 + F::new(0.27777777777777777777e-1) * t99972 + F::new(0.53958333333333333334e-1) * t99974 + F::new(0.625e-1) * t99977 + F::new(0.625e-1) * t99979 - F::new(0.16666666666666666667e0) * t99981;
    t101681
}
