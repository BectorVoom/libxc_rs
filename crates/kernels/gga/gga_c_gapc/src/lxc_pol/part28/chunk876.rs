//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 876/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk876<F: Float>(t277: F, t5294: F, t291: F, t3694: F, t3439: F, t3273: F, t9529: F, t1092: F, t2486: F, t7182: F, t906: F, t904: F) -> (F, F, F, F, F) {
    let t9957 = t277 * t5294;
    let t9958 = t3694 * t291;
    let t9959 = t9958 * t3439;
    let t9960 = t9957 * t9959;
    let t9962 = t9529 * t3273;
    let t9964 = t1092 * t2486;
    let t9966 = t7182 * t906;
    let t9967 = t904 * t9966;
    (t9959, t9960, t9962, t9964, t9967)
}
