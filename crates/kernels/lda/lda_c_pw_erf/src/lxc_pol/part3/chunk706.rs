//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 706/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk706<F: Float>(t1733: F, t2211: F, t2764: F, t2772: F, t2779: F, t2799: F, t2801: F, t2811: F, t4425: F, t4427: F, t4430: F, t4435: F, t4439: F, t4441: F, t4449: F, t4454: F, t4455: F, t4457: F, t777: F) -> F {
    let t4459 = -t4425 - F::cast_from(0.0005811348303577384_f64) * t4427 - F::cast_from(3.0_f64) * t2764 * t4430 + F::cast_from(0.19816831758676853_f64) * t4435 + F::cast_from(0.001355981270834723_f64) * t4439 + F::cast_from(3.0_f64) * t1733 * t4441 - t777 * t2799 + F::cast_from(2.0_f64) * t777 * t2779 + F::cast_from(3.0_f64) * t2211 * t2801 + F::cast_from(6.0_f64) * t4449 * t2811 - F::cast_from(0.054045904796391424_f64) * t2772 + t4454 + F::cast_from(0.039914113367515366_f64) * t4455 - F::cast_from(0.05321881782335382_f64) * t4457;
    t4459
}
