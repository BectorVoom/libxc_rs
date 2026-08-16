//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 990/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk990(t10833: f64, t5493: f64, t1095: f64, t3564: f64, t1940: f64, t10769: f64, t10801: f64, t10803: f64, t10807: f64, t10812: f64, t10814: f64, t10816: f64, t10823: f64, t10827: f64, t5852: f64, t5859: f64, t7332: f64, t7357: f64, t9148: f64, t9185: f64, t9192: f64) -> (f64, f64, f64, f64) {
    let t10834 = t10833 * t5493;
    let t10841 = t3564 * t1095;
    let t10842 = t10841 * t1940;
    let t10859 = 0.264729375e1_f64 * t10801 - 0.52945875e1_f64 * t10803 + 0.3529725e1_f64 * t10807 - t5852 + 0.20659e1_f64 * t7357 - 0.1549425e1_f64 * t9148 + 0.1549425e1_f64 * t10769 - 0.157790625e0_f64 * t10812 + 0.94674375e0_f64 * t10814 + 0.6311625e0_f64 * t10816 - t5859 + 0.104195e1_f64 * t7332 - 0.62517e0_f64 * t9185 - 0.62517e0_f64 * t9192 + 0.937755e0_f64 * t10823 + 0.312585e0_f64 * t10827;
    (t10834, t10841, t10842, t10859)
}
