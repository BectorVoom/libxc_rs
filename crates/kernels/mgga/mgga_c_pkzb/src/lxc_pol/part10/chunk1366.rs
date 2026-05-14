//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1366/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1366<F: Float>(t3046: F, t2215: F, t2209: F, t9771: F, t3041: F, t7958: F, t2203: F, t836: F, t9798: F, t9776: F, t9805: F, t18445: F, t18451: F, t22230: F, t22233: F, t22236: F, t27308: F) -> (F, F, F, F, F, F, F, F) {
    let t27310 = t3046 * t3046;
    let t27311 = t2215 * t27310;
    let t27318 = t9771 * t2209;
    let t27320 = t3041 * t7958;
    let t27323 = t2203 * t9798 * t836;
    let t27325 = t9776 * t2209;
    let t27327 = t9805 * t2209;
    let t27329 = 0.16504875e0 * t27308 + 0.16504875e0 * t27311 - 0.14717333333333333333e1 * t18445 + 0.27595e0 * t18451 - 0.18786444444444444444e1 * t22230 + 0.16102666666666666667e1 * t22233 - 0.60385e0 * t22236 + 0.19419375e1 * t27318 - 0.258925e1 * t27320 - 0.258925e1 * t27323 - 0.1294625e1 * t27325 - 0.412621875e-1 * t27327;
    (t27310, t27311, t27318, t27320, t27323, t27325, t27327, t27329)
}
