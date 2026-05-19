//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1384/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1384<F: Float>(t11810: F, t11813: F, t15509: F, t15510: F, t15511: F, t15516: F, t15518: F, t15520: F, t15522: F, t15524: F, t15526: F, t15527: F, t15699: F, t9338: F, t9340: F) -> F {
    let t18173 = F::cast_from(0.003030876351851852_f64) * t11810 - F::cast_from(0.027012345679012346_f64) * t11813 - t15509 + t15510 + t15511 - t15516 + t15518 - t15520 - t15522 + t15524 - t15526 + t15527 - t15699 + F::cast_from(0.033245444444444446_f64) * t9338 + F::cast_from(0.19947266666666666_f64) * t9340;
    t18173
}
