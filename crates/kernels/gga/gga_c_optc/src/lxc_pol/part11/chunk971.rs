//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 971/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk971<F: Float>(t17361: F, t17425: F, t17429: F, t17431: F, t17433: F, t17435: F, t17438: F, t17655: F, t17658: F, t17746: F, t17753: F, t17755: F, t17758: F, t17761: F, t17764: F, t17777: F, t2935: F, t2974: F, t3035: F, t3059: F, t402: F) -> F {
    let t17780 = -t17425 - t17429 - t17431 - t17433 - t17435 + t17438 - F::cast_from(0.19751789702565206229e-1_f64) * t17361 + t17655 - t17658 + t17746 - F::cast_from(0.35089340384731224426e1_f64) * t3035 * t17755 + F::cast_from(0.51947267698127589897e2_f64) * t3059 * t17758 - F::new(6.0) * t2935 * t17761 + F::cast_from(0.96494049533612093922e2_f64) * t2974 * t17764 + t17753 - F::new(0.3109e-1) * t17777 * t402;
    t17780
}
