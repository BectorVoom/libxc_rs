//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1162/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1162<F: Float>(t4703: F, t835: F, t1518: F, t211: F, t2467: F, t12728: F, t2076: F, t5175: F, t2104: F, t6215: F, t1284: F, t514: F, t548: F, t6866: F, t2137: F, t5211: F) -> (F, F, F, F, F, F, F, F) {
    let t17100 = 4.0 / 15.0 * t4703 * t835;
    let t17102 = t211 * t1518 * t2467;
    let t17103 = 8.0 / 135.0 * t17102;
    let t17104 = 8.0 / 45.0 * t12728;
    let t17105 = t2076 * t5175;
    let t17106 = 8.0 / 9.0 * t17105;
    let t17107 = t2104 * t6215;
    let t17108 = 16.0 / 45.0 * t17107;
    let t17109 = t1284 * t6215;
    let t17110 = 16.0 / 45.0 * t17109;
    let t17112 = t548 * t514 * t6866;
    let t17113 = 16.0 / 45.0 * t17112;
    let t17114 = t5211 * t2137;
    (t17100, t17103, t17104, t17106, t17108, t17110, t17113, t17114)
}
