//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1167/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1167<F: Float>(t1955: F, t8930: F, t10094: F, t10096: F, t12160: F, t12254: F, t12282: F, t12289: F, t13705: F, t13708: F, t13710: F, t13712: F, t13715: F, t13717: F, t13720: F, t13722: F, t13724: F, t13726: F, t13729: F, t13731: F, t13734: F, t2061: F, t25: F, t3587: F, t589: F) -> (F, F) {
    let t13736 = t8930 * t1955;
    let t13740 = F::new(0.017777777777777778) * t2061 * t3587 * t12160 + F::new(0.013333333333333334) * t25 * t589 * t12282 - F::new(0.08) * t2061 * t589 * t12289 + F::new(0.035555555555555556) * t25 * t3587 * t12254 + F::new(0.08) * t13705 - F::new(0.5038833333333333) * t13708 + F::new(0.09597777777777777) * t13710 + F::new(0.21595) * t13712 - t13715 + F::new(0.07198333333333333) * t13717 - F::new(0.4319) * t13720 - F::new(0.07198333333333333) * t13722 - F::new(0.14396666666666666) * t13724 + F::new(1.5836333333333332) * t13726 - F::new(0.4319) * t13729 + F::new(0.03732469135802469) * t13731 + F::new(0.14396666666666666) * t13734 + F::new(1.1757277777777777) * t13736 - F::new(0.07198333333333333) * t10094 + F::new(0.023994444444444443) * t10096;
    (t13736, t13740)
}
