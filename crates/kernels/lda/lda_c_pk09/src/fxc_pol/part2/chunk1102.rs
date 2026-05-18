//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1102/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1102<F: Float>(t2740: F, t6253: F, t1672: F, t2943: F, t2762: F, t2769: F, t2765: F, t12147: F, t455: F, t7353: F, t7395: F, t7400: F, t7402: F, t7411: F, t7413: F, t7415: F, t7418: F, t7421: F, t7422: F, t7426: F, t7430: F) -> F {
    let t12150 = t2740 * t6253;
    let t12154 = t2943 * t1672;
    let t12156 = t2762 * t1672;
    let t12161 = t2769 * t1672;
    let t12164 = t2765 * t1672;
    let t12169 = -F::new(2.2140749178833072) * t12147 * t455 + F::new(0.9941357652469939) * t12150 + F::new(0.8091720650647759) * t7353 + F::new(0.7380249726277691) * t7395 + t7400 - t7402 + F::new(0.7380249726277691) * t12154 - F::new(6.496391258193384) * t12156 - F::new(6.211752672544321) * t7411 - F::new(1.6457779058161184) * t7413 + F::new(0.8091720650647759) * t7415 - F::new(0.6268457032291772) * t12161 + F::new(0.7380249726277691) * t7418 - F::new(1.6457779058161184) * t12164 - t7421 - F::new(3.7610742193750633) * t7422 + F::new(1.8805371096875316) * t7426 - F::new(2.2140749178833072) * t7430;
    t12169
}
