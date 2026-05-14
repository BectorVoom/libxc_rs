//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1049/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1049<F: Float>(t1353: F, t23767: F, t31543: F, t1061: F, t424: F, t481: F, t6603: F, t7974: F, t10167: F, t1358: F, t30204: F, t6525: F, t7967: F, t3338: F, t447: F, t2366: F) -> (F, F, F, F, F, F, F) {
    let t31546 = 0.63233348079280332442e-2 * t23767 * t31543 * t1353;
    let t31548 = t481 * t1061 * t424;
    let t31551 = 0.56910013271352299198e-1 * t31548 * t6603 * t7974;
    let t31552 = t1358 * t10167;
    let t31553 = 0.94850022118920498665e-2 * t31552;
    let t31555 = t6525 * t30204 * t7967;
    let t31556 = 0.47425011059460249332e-2 * t31555;
    let t31557 = t3338 * t447;
    let t31558 = t2366 * t31557;
    (t31546, t31548, t31551, t31553, t31556, t31557, t31558)
}
