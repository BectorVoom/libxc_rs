//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3174/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3174<F: Float>(t56159: F, t56163: F, t56167: F, t58029: F, t58032: F, t58035: F, t58038: F, t58041: F, t58044: F, t58046: F, t58048: F, t58051: F) -> F {
    let t58504 = F::cast_from(0.53814000000000000001e1_f64) * t56159 + F::cast_from(0.59793333333333333334e0_f64) * t56163 + F::new(0.71752e1) * t56167 + F::new(0.147882e1) * t58029 + F::cast_from(0.10954222222222222222e0_f64) * t58032 - F::cast_from(0.49293999999999999999e0_f64) * t58035 + F::cast_from(0.427258125e1_f64) * t58038 - F::cast_from(0.230371875e0_f64) * t58041 - F::new(0.28483875e1) * t58044 - F::new(0.28483875e1) * t58046 - F::new(0.9494625e0) * t58048 + F::new(0.46074375e0) * t58051;
    t58504
}
