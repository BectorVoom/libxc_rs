//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 817/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk817<F: Float>(t24049: F, t24050: F, t24058: F, t24060: F, t24061: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F, t28102: F, t28104: F) -> F {
    let t29285 = F::cast_from(0.80745512188280781706e-3_f64) * t26272 + t28085 / F::cast_from(384.0_f64) - t24049 + t24050 + F::cast_from(0.56521858531796547194e-2_f64) * t26295 + t28089 / F::cast_from(768.0_f64) - t28091 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t28093 - t28095 / F::cast_from(192.0_f64) - t28097 / F::cast_from(96.0_f64) + F::cast_from(0.48447307312968469024e-2_f64) * t28102 + t24058 + t24060 + t24061 + t28104 / F::cast_from(96.0_f64);
    t29285
}
