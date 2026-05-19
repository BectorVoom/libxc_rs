//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1156/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1156<F: Float>(t2389: F, t6700: F, t6696: F, t1441: F, t9264: F, t1429: F, t2365: F, t2366: F, t6393: F, t21074: F, t901: F, t20675: F, t9538: F) -> (F, F, F, F, F, F) {
    let t31213 = F::cast_from(0.11916829983950142223e0_f64) * t6700 * t2389;
    let t31215 = F::cast_from(0.11916829983950142223e0_f64) * t6696 * t2389;
    let t31216 = t1441 * t9264;
    let t31291 = F::cast_from(0.29792074959875355558e-1_f64) * t1429 * t2365 * t2366 * t6393;
    let t31299 = F::cast_from(0.29792074959875355558e-1_f64) * t21074 * t901;
    let t31346 = t20675 * t9538;
    (t31213, t31215, t31216, t31291, t31299, t31346)
}
