//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1292/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1292<F: Float>(t1858: F, t3431: F, t5679: F, t7682: F, t8792: F, t2628: F, t8521: F, t2009: F, t2021: F, t2028: F, t28529: F, t33205: F, t33206: F, t33210: F, t33212: F, t33215: F, t33218: F, t33221: F, t33223: F, t33225: F, t33228: F, t33231: F) -> F {
    let t33232 = t1858 * t3431;
    let t33238 = F::cast_from(0.21450293971110256002e1_f64) * t5679 * t8792 * t7682;
    let t33239 = t8521 * t2628;
    let t33240 = F::cast_from(0.59584149919750711116e-1_f64) * t33239;
    let t33241 = -t33205 - F::cast_from(0.79445533226334281486e-1_f64) * t33206 * t2028 - t33210 - t33212 + t28529 + t33215 - t33218 + t33221 - t33223 - t33225 + t33228 - t33231 - F::cast_from(0.71500979903700853338e0_f64) * t2021 * t33232 * t2009 - t33238 - t33240;
    t33241
}
