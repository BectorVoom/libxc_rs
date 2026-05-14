//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1023/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1023<F: Float>(t4516: F, t951: F, t13812: F, t4506: F, t13771: F, t13773: F, t4515: F, t12414: F, t4523: F, t13804: F, t4522: F, t13808: F, t3589: F, t4048: F, t581: F, t11753: F) -> (F, F, F, F, F, F, F, F) {
    let t13813 = t4516 * t951;
    let t13816 = 8.0 / 3.0 * t4506 * t13812 * t13813;
    let t13819 = 32.0 / 15.0 * t13771 * t4515 * t13773;
    let t13821 = 8.0 / 9.0 * t12414 * t4523;
    let t13824 = 4.0 / 9.0 * t4506 * t4522 * t13804;
    let t13827 = 4.0 / 9.0 * t4506 * t4522 * t13808;
    let t13829 = t4048 * t581 * t3589;
    let t13832 = 32.0 / 27.0 * t4506 * t13829 * t13813;
    let t13846 = 0.0016792592592592592 * t11753;
    (t13813, t13816, t13819, t13821, t13824, t13827, t13832, t13846)
}
