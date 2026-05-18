//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 602/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk602<F: Float>(t2675: F, t2680: F, t2683: F, t2685: F, t117: F, t118: F, t123: F, t125: F, t1328: F, t1333: F, t1337: F, t1349: F, t1352: F, t1356: F, t1360: F, t2323: F, t2327: F, t2331: F, t2338: F, t2454: F) -> (F, F) {
    let t2687 = t2675 + t2680 + t2683 + t2685;
    let t2692 = -t1328 + F::new(0.06301081444628223) * t2323 + t1333 + t1337 - F::new(0.031505407223141116) * t2454 * t118 - F::new(0.06301081444628223) * t2327 - F::new(0.003950778065781896) * t2331 - t1349 - t1352 - t1356 - t1360 + F::new(0.017961351015381915) * t2338 - F::new(0.005388405304614574) * t123 * t125 * t2687 * t117;
    (t2687, t2692)
}
