//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1443/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443<F: Float>(t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F) -> F {
    let t41371 = -F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t41341 - t41344 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) * t41347 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t41350 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t41353 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41356 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41359 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t41361 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t41363 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41365 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41367 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t41369;
    t41371
}
