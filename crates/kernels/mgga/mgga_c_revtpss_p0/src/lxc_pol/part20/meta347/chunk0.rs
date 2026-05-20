//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1275/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1275<F: Float>(t17656: F, t3604: F, t3362: F, t471: F, t2251: F, t1285: F, t12865: F, t372: F, t5302: F, t3588: F, t3603: F, t15904: F, t3623: F) -> (F, F, F, F, F, F) {
    let t17657 = t3604 * t17656;
    let t17687 = t471 * t3362;
    let t17688 = t17687 * t2251;
    let t17693 = t1285 * t12865;
    let t17694 = t372 * t5302;
    let t17703 = t3603 * t3588;
    let t17708 = t3623 * t15904;
    (t17657, t17688, t17693, t17694, t17703, t17708)
}
