//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1040/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1040<F: Float>(t10590: F, t791: F, t10577: F, t796: F, t238: F, t4131: F, t801: F, t1329: F, t3309: F, t242: F, t4135: F, t4104: F, t779: F, t10547: F, t226: F, t6614: F, t6616: F, t8706: F, t8846: F, t8847: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10591 = t10590 * t791;
    let t10593 = t796 * t10577;
    let t10598 = t238 * t801 * t4131;
    let t10600 = t1329 * t3309;
    let t10602 = t238 * t242 * t10600;
    let t10605 = t238 * t801 * t4135;
    let t10607 = t779 * t4104;
    let t10609 = t238 * t242 * t10607;
    let t10611 = t226 * t10547;
    let t10613 = t238 * t242 * t10611;
    let t10615 = 0.15358125e0 * t10591 + 0.3071625e0 * t10593 - t6614 + 0.27385555555555555556e0 * t6616 + 0.5477111111111111111e0 * t8706 - t8846 - t8847 - 0.16431333333333333333e0 * t10598 + 0.49294e0 * t10602 - 0.16431333333333333333e0 * t10605 + 0.24647e0 * t10609 + 0.24647e0 * t10613;
    (t10591, t10593, t10598, t10600, t10602, t10605, t10607, t10609, t10611, t10613, t10615)
}
