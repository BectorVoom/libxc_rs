//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1045/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1045<F: Float>(t31548: F, t6603: F, t7974: F, t10167: F, t1358: F, t30204: F, t6525: F, t7967: F, t10253: F, t2312: F, t21154: F, t2268: F, t25775: F, t10160: F, t1349: F, t25730: F, t4261: F, t9074: F) -> (F, F, F, F, F, F, F) {
    let t31551 = 0.56910013271352299198e-1 * t31548 * t6603 * t7974;
    let t31552 = t1358 * t10167;
    let t31553 = 0.94850022118920498665e-2 * t31552;
    let t31555 = t6525 * t30204 * t7967;
    let t31556 = 0.47425011059460249332e-2 * t31555;
    let t31564 = t2312 * t10253;
    let t31565 = 0.23712505529730124666e-2 * t31564;
    let t31568 = 0.17073003981405689759e1 * t2268 * t25775 * t21154;
    let t31569 = t1349 * t10160;
    let t31570 = 0.31616674039640166222e-2 * t31569;
    let t31574 = t9074 * t4261 * t25730;
    (t31551, t31553, t31556, t31565, t31568, t31570, t31574)
}
