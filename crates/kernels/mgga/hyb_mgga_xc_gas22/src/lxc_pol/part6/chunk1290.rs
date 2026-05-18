//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1290/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1290<F: Float>(t10212: F, t23894: F, t3138: F, t10158: F, t10163: F, t10205: F, t2002: F, t20218: F, t2024: F, t20252: F, t20255: F, t20258: F, t2027: F, t2028: F, t23872: F, t23905: F, t23923: F, t23925: F, t27941: F, t27955: F, t27957: F, t27962: F, t27968: F, t27976: F, t3140: F, t675: F, t684: F, t687: F, t8511: F, t8513: F, t8526: F) -> F {
    let t27979 = t3138 * t23894 * t10212;
    let t27990 = -t2024 * t2027 * t10158 * t2028 / F::new(48.0) - t684 * t687 * t27941 * t675 / F::new(16.0) - t684 * t687 * t10163 * t2002 / F::new(32.0) - t2024 * t2027 * t10163 * t2028 / F::new(24.0) - t27955 / F::new(96.0) - t8511 * t3140 * t27957 / F::new(4.0) + t27962 / F::new(24.0) + t20218 / F::new(216.0) + t20252 / F::new(144.0) - F::new(5.0) / F::new(432.0) * t20255 + t20258 / F::new(288.0) + t8526 * t3140 * t27968 / F::new(16.0) + F::new(7.0) / F::new(18.0) * t23872 * t8513 * t27957 - F::new(7.0) / F::new(216.0) * t27976 - F::new(7.0) / F::new(36.0) * t27979 - F::new(7.0) / F::new(72.0) * t8511 * t23905 * t10205 - F::new(7.0) / F::new(144.0) * t8511 * t8513 * t27968 - F::new(35.0) / F::new(216.0) * t23923 * t23925 * t27957;
    t27990
}
