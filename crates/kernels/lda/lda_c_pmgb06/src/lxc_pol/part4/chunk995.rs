//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 995/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk995<F: Float>(t3711: F, t971: F, t1066: F, t1101: F, t1026: F, t1035: F, t1041: F, t3738: F, t696: F, t8599: F, t967: F, t3705: F) -> (F, F, F, F, F) {
    let t8724 = t971 * t3711;
    let t8727 = F::new(120.0) * t1101 * t1066;
    let t8733 = F::new(36.0) * t1041 * t1026 * t1035;
    let t8737 = F::cast_from(623.3709278045327_f64) * t696 * t3738 * t8599 * t967;
    let t8738 = t971 * t3705;
    (t8724, t8727, t8733, t8737, t8738)
}
