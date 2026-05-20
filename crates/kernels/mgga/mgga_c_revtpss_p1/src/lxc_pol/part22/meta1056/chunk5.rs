//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3742/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3742<F: Float>(t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F) -> F {
    let t71162 = -F::cast_from(0.13333333333333333333e0_f64) * t68363 + F::cast_from(0.37037037037037037036e-1_f64) * t68366 - F::cast_from(0.98765432098765432098e-2_f64) * t56176 + F::cast_from(0.2962962962962962963e-1_f64) * t56183 - F::cast_from(0.22222222222222222222e-1_f64) * t56185 - F::cast_from(0.11111111111111111111e-1_f64) * t56187 - F::cast_from(0.33333333333333333333e-1_f64) * t56189 + F::cast_from(0.74074074074074074074e-2_f64) * t56209 + F::cast_from(0.37037037037037037037e-2_f64) * t56212 + F::cast_from(0.22222222222222222222e-1_f64) * t56214 - F::cast_from(0.61728395061728395062e-2_f64) * t56216 + F::cast_from(0.14814814814814814815e-1_f64) * t56228;
    t71162
}
