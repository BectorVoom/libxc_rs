//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 687/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk687(t439: f64, t6250: f64, t2604: f64, t3031: f64, t477: f64, t1966: f64, t1967: f64, t2064: f64, t4718: f64, t4721: f64, t4723: f64, t4725: f64, t4740: f64, t6229: f64, t6234: f64, t6236: f64, t6238: f64, t6240: f64, t6243: f64, t6247: f64, t6249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6252 = t439 * t6250 / 27.0_f64;
    let t6253 = t3031 * t2604;
    let t6254 = t6253 * t477;
    let t6255 = t1966 * t6254;
    let t6257 = t439 * t6255 / 5.0_f64;
    let t6258 = t1967 * t2064;
    let t6259 = t1966 * t6258;
    let t6261 = 2.0_f64 / 15.0_f64 * t439 * t6259;
    let t6262 = 4.0_f64 / 135.0_f64 * t4718 - t4721 - t4723 + t4725 + 0.06649088888888889_f64 * t4740 - t6229 - t6234 - t6236 - t6238 + t6240 + t6243 + t6247 + t6249 + t6252 - t6257 + t6261;
    (t6252, t6253, t6254, t6255, t6257, t6258, t6259, t6261, t6262)
}
