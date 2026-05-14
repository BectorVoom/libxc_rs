//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1034/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1034<F: Float>(t1212: F, t25838: F, t1528: F, t8344: F, t12935: F, t14784: F, t14785: F, t19104: F, t19543: F, t19545: F, t21804: F, t25699: F, t25701: F, t25704: F, t25711: F, t25607: F, t25612: F, t25615: F, t25627: F, t25629: F, t25632: F, t25634: F, t25743: F, t25745: F, t25748: F, t25750: F) -> (F, F, F, F) {
    let t27511 = t25838 * t1212;
    let t27516 = t8344 * t1528;
    let t27545 = 0.69463333333333333333e-1 * t25699 + 0.23154444444444444445e-1 * t25701 - 0.104195e0 * t25704 - 0.11577222222222222222e0 * t12935 + t21804 - 0.68863333333333333332e0 * t19104 - t14784 - t14785 - 0.23154444444444444445e0 * t19543 + 0.27785333333333333334e0 * t19545 + 0.20839e0 * t25711;
    let t27569 = 0.41318e1 * t25607 - 0.34431666666666666667e0 * t25612 + 0.103295e1 * t25615 - 0.17648625e1 * t25632 - 0.157790625e0 * t25743 + 0.6311625e0 * t25745 + 0.31558125e0 * t25748 + 0.6311625e0 * t25750 + 0.3529725e1 * t25634 + 0.264729375e1 * t25627 - 0.3529725e1 * t25629;
    (t27511, t27516, t27545, t27569)
}
