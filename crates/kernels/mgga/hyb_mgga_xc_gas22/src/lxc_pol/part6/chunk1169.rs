//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1169/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1169<F: Float>(t2813: F, t3636: F, t462: F, t1523: F, t7482: F, t3616: F, t7554: F, t10: F, t1107: F, t9369: F, t7269: F, t7516: F, t7242: F, t1057: F, t9327: F, t9370: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25954 = t462 * t3636 * t2813;
    let t25957 = t462 * t1523 * t7482;
    let t25959 = t3616 * t7554;
    let t25962 = t9369 * t10 * t1107;
    let t25964 = t3616 * t7269;
    let t25966 = t3616 * t7516;
    let t25968 = t3616 * t7242;
    let t25973 = t1057 * t9327;
    let t25975 = t1057 * t9370;
    (t25954, t25957, t25959, t25962, t25964, t25966, t25968, t25973, t25975)
}
