//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1026/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1026<F: Float>(t10105: F, t1052: F, t1960: F, t3418: F, t6553: F, t10283: F, t2497: F, t42520: F, t42523: F, t42904: F, t44215: F, t44217: F, t44221: F, t44223: F, t44225: F, t44228: F, t44231: F, t44232: F, t44234: F, t44236: F, t44238: F, t44239: F) -> (F, F, F) {
    let t44242 = F::cast_from(2.0_f64) * t1960 * t1052 * t10105;
    let t44243 = t6553 * t3418;
    let t44244 = F::cast_from(2.0_f64) * t44243;
    let t44245 = t10283 * t2497;
    let t44246 = F::cast_from(2.0_f64) * t44245;
    let t44247 = F::cast_from(4.0_f64) * t44215 + F::cast_from(4.0_f64) * t44217 - t44221 + t44223 + t44225 - t44228 + t42520 + t44231 - t42523 - t44232 - t44234 - t42904 + t44236 + t44238 - t44239 + t44242 + t44244 + t44246;
    (t44244, t44246, t44247)
}
