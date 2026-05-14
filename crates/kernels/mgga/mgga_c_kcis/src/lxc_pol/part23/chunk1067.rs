//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1067/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1067<F: Float>(t7908: F, t94585: F, t27484: F, t7895: F, t1014: F, t27332: F, t27424: F, t3728: F, t27464: F, t3245: F, t7928: F, t12504: F, t491: F, t27543: F, t3733: F, t4244: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94638 = t7908 * t94585;
    let t94651 = t7895 * t27484;
    let t94656 = t1014 * t27332;
    let t94662 = t3728 * t27424;
    let t94664 = t1014 * t27464;
    let t94669 = t3245 * t7928;
    let t94743 = t12504 * t491;
    let t94748 = t3733 * t27543;
    let t94754 = t4244 * t491;
    (t94638, t94651, t94656, t94662, t94664, t94669, t94743, t94748, t94754)
}
