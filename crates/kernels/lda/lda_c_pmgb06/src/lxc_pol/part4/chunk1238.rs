//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1238/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1238<F: Float>(t16305: F, t2470: F, t3223: F, t1447: F, t6120: F, t439: F, t4766: F, t6550: F, t2477: F, t3213: F, t10687: F, t10690: F, t16293: F, t16295: F, t16297: F, t16299: F, t16300: F, t16301: F, t16302: F, t16303: F) -> (F, F, F, F, F, F) {
    let t16306 = t16305 / F::cast_from(135.0_f64);
    let t16307 = t3223 * t2470;
    let t16308 = F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t16307;
    let t16309 = t1447 * t6120;
    let t16310 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16309;
    let t16313 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t439 * t6550 * t4766;
    let t16314 = t3213 * t2477;
    let t16315 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t16314;
    let t16316 = -t16293 - t16295 - t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 - t10687 + t10690 - t16308 + t16310 - t16313 - t16315;
    (t16306, t16308, t16310, t16313, t16315, t16316)
}
