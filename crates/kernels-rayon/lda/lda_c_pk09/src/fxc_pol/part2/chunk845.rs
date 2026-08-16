//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 845/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk845(t3820: f64, t7731: f64, t7608: f64, t1067: f64, t2222: f64, t4705: f64, t3823: f64, t4581: f64, t4612: f64, t4614: f64, t709: f64, t7578: f64, t7590: f64, t7598: f64, t7602: f64, t8651: f64) -> f64 {
    let t8657 = t3820 * t7731;
    let t8663 = t3820 * t7608;
    let t8669 = t2222 * t1067;
    let t8675 = t4705 * t7608;
    let t8677 = -19.489173774580152_f64 * t8651 * t709 - 3.7610742193750633_f64 * t8657 - 7.5221484387501265_f64 * t3823 * t7598 - 3.7610742193750633_f64 * t3823 * t7602 - 3.7610742193750633_f64 * t8663 - 3.7610742193750633_f64 * t3823 * t7590 - 7.5221484387501265_f64 * t3823 * t7578 + 1.9882715304939877_f64 * t8669 + 37.27051603526593_f64 * t4581 * t7598 + 18.635258017632964_f64 * t4581 * t7602 + 18.635258017632964_f64 * t8675 + t4612 + t4614;
    t8677
}
