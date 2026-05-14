//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1017/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1017<F: Float>(t325: F, t4672: F, t4606: F, t4690: F, t1953: F, t3618: F, t557: F, t817: F, t1349: F, t3609: F, t1955: F, t8930: F, t10094: F, t10096: F, t12160: F, t12254: F, t12282: F, t12289: F, t13705: F, t13708: F, t13710: F, t13712: F, t13715: F, t13717: F, t13720: F, t13722: F, t2061: F, t25: F, t3587: F, t589: F) -> (F, F, F, F, F, F, F) {
    let t13724 = t325 * t4672;
    let t13726 = t4606 * t4690;
    let t13729 = t1953 * t557 * t3618;
    let t13731 = t1953 * t817;
    let t13734 = t1953 * t1349 * t3609;
    let t13736 = t8930 * t1955;
    let t13740 = 0.017777777777777778 * t2061 * t3587 * t12160 + 0.013333333333333334 * t25 * t589 * t12282 - 0.08 * t2061 * t589 * t12289 + 0.035555555555555556 * t25 * t3587 * t12254 + 0.08 * t13705 - 0.5038833333333333 * t13708 + 0.09597777777777777 * t13710 + 0.21595 * t13712 - t13715 + 0.07198333333333333 * t13717 - 0.4319 * t13720 - 0.07198333333333333 * t13722 - 0.14396666666666666 * t13724 + 1.5836333333333332 * t13726 - 0.4319 * t13729 + 0.03732469135802469 * t13731 + 0.14396666666666666 * t13734 + 1.1757277777777777 * t13736 - 0.07198333333333333 * t10094 + 0.023994444444444443 * t10096;
    (t13724, t13726, t13729, t13731, t13734, t13736, t13740)
}
