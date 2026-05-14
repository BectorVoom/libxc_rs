//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1026/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1026<F: Float>(t13852: F, t13872: F, t173: F, t184: F, t199: F, t1405: F, t1960: F, t13796: F, t13800: F, t13802: F, t13807: F, t13811: F, t13816: F, t13819: F, t13821: F, t13824: F, t13827: F, t13832: F) -> (F, F, F) {
    let t13877 = 2.0 / 15.0 * t173 * (t13852 + t13872) * t184 * t199;
    let t13879 = 4.0 / 5.0 * t1960 * t1405;
    let t13880 = t13796 + t13800 + t13802 + t13807 + t13811 + t13816 - t13819 - t13821 - t13824 - t13827 - t13832 + t13877 + t13879;
    (t13877, t13879, t13880)
}
