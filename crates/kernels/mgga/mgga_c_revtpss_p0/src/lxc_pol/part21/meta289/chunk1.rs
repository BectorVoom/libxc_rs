//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1528/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1528<F: Float>(t10345: F, t10357: F, t10361: F, t10364: F, t10369: F, t10373: F, t10376: F, t10379: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> F {
    let t10380 = -F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t10345 * t49 + F::cast_from(220.0_f64) / F::cast_from(9.0_f64) * t2270 * t617 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t614 * t2276 - F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t614 * t2279 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t44 * t10357 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t10361 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t10364 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t56 * t10369 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t10373 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t10376 + t10379;
    t10380
}
