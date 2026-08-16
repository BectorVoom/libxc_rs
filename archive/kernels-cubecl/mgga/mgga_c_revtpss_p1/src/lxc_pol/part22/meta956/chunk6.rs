//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3206/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206<F: Float>(t1469: F, t627: F, t72: F, t13389: F, t13406: F, t13409: F, t13414: F, t1471: F, t1494: F, t21686: F, t21687: F, t21805: F, t2251: F, t2252: F, t2259: F, t2260: F, t2263: F, t4186: F, t4188: F, t4191: F, t4196: F, t4238: F, t5854: F, t5869: F, t608: F, t6977: F, t85: F) -> F {
    let t60823 = t1469 * t627 * t72;
    let t60829 = -t4196 * t4238 / F::cast_from(3.0_f64) - t2260 * t5869 / F::cast_from(12.0_f64) - t2263 * t5869 / F::cast_from(6.0_f64) - t608 * t21805 / F::cast_from(6.0_f64) - t13406 * t1494 / F::cast_from(6.0_f64) - t13409 * t1494 / F::cast_from(3.0_f64) - t4188 * t4238 / F::cast_from(3.0_f64) - t13414 * t1494 / F::cast_from(6.0_f64) - t4191 * t4238 / F::cast_from(3.0_f64) - t1471 * t13389 / F::cast_from(6.0_f64) - t2259 * t5854 * t85 / F::cast_from(12.0_f64) - t2252 * t5869 / F::cast_from(12.0_f64) - t2251 * t5854 * t85 / F::cast_from(12.0_f64) - t60823 * t21687 / F::cast_from(3.0_f64) - t21686 * t6977 * t4186 / F::cast_from(3.0_f64);
    t60829
}
