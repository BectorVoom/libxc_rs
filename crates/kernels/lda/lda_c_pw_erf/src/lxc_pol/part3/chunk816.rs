//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk816<F: Float>(t133: F, t5506: F, t5521: F, t3280: F, t3284: F, t3322: F, t3348: F, t3361: F, t5550: F, t5570: F, t5577: F, t5588: F, t5591: F, t5609: F) -> F {
    let t5660 = t133 * t5506;
    let t5663 = F::cast_from(1.1495033333333333_f64) * t133 * t5521;
    let t5666 = -F::cast_from(1.724255_f64) * t3361 + t3280 - t3284 - t5570 - t3348 - t5577 + t5588 + t5591 - F::cast_from(0.7663355555555555_f64) * t5660 + t5663 - F::cast_from(1.724255_f64) * t133 * t5550 - t5609 - t3322;
    t5666
}
