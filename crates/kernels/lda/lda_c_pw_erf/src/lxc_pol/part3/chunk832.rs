//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 832/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk832<F: Float>(t2281: F, t668: F, t267: F, t3682: F, t3684: F, t3706: F, t4534: F, t4535: F, t4549: F, t4550: F, t4551: F, t4552: F, t4553: F, t4554: F, t4555: F, t4563: F, t4566: F, t5806: F, t5833: F) -> F {
    let t5837 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2281 * t668;
    let t5838 = -t4534 + t4535 + F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t5806 + F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t3682 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t3684 - t3706 - t5833 * t267 / F::cast_from(15.0_f64) - t5837 + t4549 - t4550 - t4551 + t4552 - t4553 - t4554 - t4555 + t4563 - t4566;
    t5838
}
