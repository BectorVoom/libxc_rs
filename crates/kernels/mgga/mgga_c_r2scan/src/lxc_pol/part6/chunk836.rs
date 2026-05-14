//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 836/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk836<F: Float>(t5986: F, t5834: F, t5945: F, t5950: F, t5952: F, t5955: F, t5959: F, t5963: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5980: F, t5982: F, t5985: F) -> (F,) {
    let t5987 = 240.0 * t5986;
    let t5988 = t5945 - t5950 - 0.60030643514799999999e-2 * t5952 - 0.1200612870296e-1 * t5955 + t5959 + t5963 - t5966 + 0.65061487801810439052e-1 * t5968 - 0.96319466275353142157e0 * t5970 - 0.3903689268108626343e0 * t5972 - t5975 + 24.0 * t5976 - 0.33872559466666666666e-2 * t5978 + t5834 - 24.0 * t5980 - 60.0 * t5982 + t5985 - t5987;
    (t5988,)
}
