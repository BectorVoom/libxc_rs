//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 905/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk905<F: Float>(t2104: F, t6215: F, t1284: F, t514: F, t548: F, t6866: F, t2137: F, t5211: F, t1518: F, t185: F, t2498: F, t1318: F, t3899: F, t6992: F, t519: F, t5237: F, t6336: F) -> (F, F, F, F, F, F, F) {
    let t17107 = t2104 * t6215;
    let t17109 = t1284 * t6215;
    let t17112 = t548 * t514 * t6866;
    let t17114 = t5211 * t2137;
    let t17117 = t185 * t1518 * t2498;
    let t17123 = t1318 * t3899 * t6992;
    let t17156 = t519 * t5237 * t6336;
    (t17107, t17109, t17112, t17114, t17117, t17123, t17156)
}
