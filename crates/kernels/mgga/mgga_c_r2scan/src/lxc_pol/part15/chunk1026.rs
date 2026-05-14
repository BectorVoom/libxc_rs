//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1026/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1026<F: Float>(t37841: F, t2833: F, t545: F, t3300: F, t10875: F, t11744: F, t146: F, t2206: F, t2832: F, t3305: F, t10760: F, t22820: F, t25697: F, t37848: F, t37851: F, t37823: F, t37834: F, t37835: F, t37838: F, t37843: F) -> (F,) {
    let t39738 = 0.13506635798907349461e1 * t37841;
    let t39739 = t545 * t2833;
    let t39740 = t39739 * t3300;
    let t39742 = t11744 * t10875;
    let t39745 = t146 * t2206 * t2832;
    let t39746 = t39745 * t3305;
    let t39749 = t22820 * t10760 * t25697;
    let t39752 = 0.84755945902752848174e0 * t37848;
    let t39753 = 0.25426783770825854452e1 * t37851;
    let t39754 = t37823 + t37834 + 0.58544643236296698112e-1 * t37835 + 0.45022119329691164872e0 * t37838 + t39738 - 0.86682217400542685632e-1 * t39740 - 0.43341108700271342816e-1 * t39742 - 0.2600466522016280569e0 * t39746 + 0.13099107994629972538e-1 * t39749 + 0.27439371595564631661e-2 * t37843 - t39752 - t39753;
    (t39754,)
}
