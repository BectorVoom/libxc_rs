//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 878/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk878<F: Float>(t334: F, t8218: F, t913: F, t904: F, t907: F, t319: F, t4606: F, t5021: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F) -> (F, F, F) {
    let t8221 = F::new(6.0) * t913 * t8218 * t334;
    let t8224 = F::new(48.24547296645331) * t904 * t8218 * t907;
    let t8238 = F::new(1.0) * t319 * (-F::new(2.109916666666667) * t8141 + F::new(20.2552) * t8143 - F::new(7.501925925925926) * t8146 + F::new(6.564185185185186) * t8149 + F::new(3.100395061728395) * t4606 + F::new(0.06825833333333334) * t8155 - F::new(1.0921333333333334) * t8157 + F::new(1.2134814814814814) * t8159 + F::new(1.0617962962962963) * t8161 + F::new(1.3388493827160495) * t5021) * t334;
    (t8221, t8224, t8238)
}
