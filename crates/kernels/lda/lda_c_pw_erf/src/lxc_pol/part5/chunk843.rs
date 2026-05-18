//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 843/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk843<F: Float>(t1268: F, t7639: F, t7643: F, t538: F, t7647: F, t7651: F, t7655: F, t3516: F, t7635: F, t25: F, t3472: F, t3543: F, t4600: F, t7641: F, t7645: F) -> (F, F, F, F, F, F, F) {
    let t7758 = t1268 * t7639;
    let t7761 = t1268 * t7643;
    let t7764 = t538 * t7647;
    let t7767 = t538 * t7651;
    let t7770 = t538 * t7655;
    let t7773 = t3516 * t7635;
    let t7779 = F::new(0.013333333333333334) * t25 * t7758 - F::new(0.006666666666666667) * t25 * t7761 - F::new(0.04) * t25 * t7764 + F::new(0.04) * t25 * t7767 - F::new(0.006666666666666667) * t25 * t7770 - F::new(0.002962962962962963) * t25 * t7773 - t3472 - F::new(0.047988888888888886) * t4600 - t3543 + F::new(0.14396666666666666) * t7641 - F::new(0.07198333333333333) * t7645;
    (t7758, t7761, t7764, t7767, t7770, t7773, t7779)
}
