//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1167/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1167(t1955: f64, t8930: f64, t10094: f64, t10096: f64, t12160: f64, t12254: f64, t12282: f64, t12289: f64, t13705: f64, t13708: f64, t13710: f64, t13712: f64, t13715: f64, t13717: f64, t13720: f64, t13722: f64, t13724: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t2061: f64, t25: f64, t3587: f64, t589: f64) -> (f64, f64) {
    let t13736 = t8930 * t1955;
    let t13740 = 0.017777777777777778_f64 * t2061 * t3587 * t12160 + 0.013333333333333334_f64 * t25 * t589 * t12282 - 0.08_f64 * t2061 * t589 * t12289 + 0.035555555555555556_f64 * t25 * t3587 * t12254 + 0.08_f64 * t13705 - 0.5038833333333333_f64 * t13708 + 0.09597777777777777_f64 * t13710 + 0.21595_f64 * t13712 - t13715 + 0.07198333333333333_f64 * t13717 - 0.4319_f64 * t13720 - 0.07198333333333333_f64 * t13722 - 0.14396666666666666_f64 * t13724 + 1.5836333333333332_f64 * t13726 - 0.4319_f64 * t13729 + 0.03732469135802469_f64 * t13731 + 0.14396666666666666_f64 * t13734 + 1.1757277777777777_f64 * t13736 - 0.07198333333333333_f64 * t10094 + 0.023994444444444443_f64 * t10096;
    (t13736, t13740)
}
