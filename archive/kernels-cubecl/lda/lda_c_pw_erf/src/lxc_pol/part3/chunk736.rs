//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 736/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk736<F: Float>(t3627: F, t3629: F, t3631: F, t3646: F, t4013: F, t4657: F, t4659: F, t4662: F, t4663: F, t4668: F, t4673: F, t4678: F, t4682: F, t4686: F, t4691: F, t4695: F, t4699: F) -> F {
    let t4701 = t4013 + F::cast_from(0.0016792592592592592_f64) * t3627 - F::cast_from(0.0004198148148148148_f64) * t3631 + F::cast_from(0.0012594444444444445_f64) * t3646 - F::cast_from(0.0006297222222222223_f64) * t3629 + F::cast_from(0.0008396296296296296_f64) * t4657 - F::cast_from(0.0008396296296296296_f64) * t4659 + t4662 + F::cast_from(0.01385388888888889_f64) * t4663 + F::cast_from(0.002099074074074074_f64) * t4668 - F::cast_from(0.007556666666666666_f64) * t4673 - F::cast_from(0.005037777777777778_f64) * t4678 + F::cast_from(0.0012594444444444445_f64) * t4682 + F::cast_from(0.011335_f64) * t4686 + F::cast_from(0.015113333333333333_f64) * t4691 - F::cast_from(0.003778333333333333_f64) * t4695 - F::cast_from(0.003778333333333333_f64) * t4699;
    t4701
}
