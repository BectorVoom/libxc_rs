//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 921/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk921<F: Float>(t10833: F, t5493: F, t1095: F, t3564: F, t1940: F, t10769: F, t10801: F, t10803: F, t10807: F, t10812: F, t10814: F, t10816: F, t10823: F, t10827: F, t5852: F, t5859: F, t7332: F, t7357: F, t9148: F, t9185: F, t9192: F) -> (F, F, F, F) {
    let t10834 = t10833 * t5493;
    let t10841 = t3564 * t1095;
    let t10842 = t10841 * t1940;
    let t10859 = 0.264729375e1 * t10801 - 0.52945875e1 * t10803 + 0.3529725e1 * t10807 - t5852 + 0.20659e1 * t7357 - 0.1549425e1 * t9148 + 0.1549425e1 * t10769 - 0.157790625e0 * t10812 + 0.94674375e0 * t10814 + 0.6311625e0 * t10816 - t5859 + 0.104195e1 * t7332 - 0.62517e0 * t9185 - 0.62517e0 * t9192 + 0.937755e0 * t10823 + 0.312585e0 * t10827;
    (t10834, t10841, t10842, t10859)
}
