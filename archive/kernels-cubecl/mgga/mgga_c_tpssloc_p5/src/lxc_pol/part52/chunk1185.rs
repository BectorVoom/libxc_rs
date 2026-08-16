//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1185/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1185<F: Float>(t31046: F, t31050: F, t31052: F, t31055: F, t31057: F, t31060: F, t31065: F, t31067: F, t31070: F, t31072: F, t31077: F, t650: F, t6517: F, t7271: F, t8682: F) -> F {
    let t31849 = -t650 * t8682 - F::cast_from(2.0_f64) * t6517 * t7271 + t31046 + t31050 - F::cast_from(2.0_f64) * t31052 - t31055 - t31057 - t31060 - F::cast_from(2.0_f64) * t31065 - F::cast_from(2.0_f64) * t31067 - F::cast_from(2.0_f64) * t31070 - F::cast_from(2.0_f64) * t31072 - t31077;
    t31849
}
