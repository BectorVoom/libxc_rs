//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 878/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk878<F: Float>(t439: F, t6250: F, t2604: F, t3031: F, t477: F, t1966: F, t1967: F, t2064: F, t4718: F, t4721: F, t4723: F, t4725: F, t4740: F, t6229: F, t6234: F, t6236: F, t6238: F, t6240: F, t6243: F, t6247: F, t6249: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6252 = t439 * t6250 / F::new(27.0);
    let t6253 = t3031 * t2604;
    let t6254 = t6253 * t477;
    let t6255 = t1966 * t6254;
    let t6257 = t439 * t6255 / F::new(5.0);
    let t6258 = t1967 * t2064;
    let t6259 = t1966 * t6258;
    let t6261 = F::new(2.0) / F::new(15.0) * t439 * t6259;
    let t6262 = F::new(4.0) / F::new(135.0) * t4718 - t4721 - t4723 + t4725 + F::cast_from(0.06649088888888889_f64) * t4740 - t6229 - t6234 - t6236 - t6238 + t6240 + t6243 + t6247 + t6249 + t6252 - t6257 + t6261;
    (t6252, t6253, t6254, t6255, t6257, t6258, t6259, t6261, t6262)
}
