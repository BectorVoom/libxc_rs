//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 453/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk453<F: Float>(t44: F, t2225: F, t2300: F, t2408: F, t2437: F, t7: F, t2140: F, t413: F, t1165: F, t1173: F, t1693: F, t1694: F, t1695: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t2439 = t2225 + t2300 + t2408 + t2437;
    let t2440 = t7 * t2439;
    let t2444 = piecewise3::<F>(t45, F::new(0.0), F::new(2.0) * t44 * t2140);
    let t2445 = t2444 * t413;
    let t2447 = -t1165 + t1693 + t1694 - t1695 + t1173;
    (t2439, t2440, t2444, t2445, t2447)
}
