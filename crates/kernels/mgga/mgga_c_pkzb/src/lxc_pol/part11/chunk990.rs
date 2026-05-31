//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 990/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk990<F: Float>(t10833: F, t5493: F, t1095: F, t3564: F, t1940: F, t10769: F, t10801: F, t10803: F, t10807: F, t10812: F, t10814: F, t10816: F, t10823: F, t10827: F, t5852: F, t5859: F, t7332: F, t7357: F, t9148: F, t9185: F, t9192: F) -> (F, F, F, F) {
    let t10834 = t10833 * t5493;
    let t10841 = t3564 * t1095;
    let t10842 = t10841 * t1940;
    let t10859 = F::cast_from(0.264729375e1_f64) * t10801 - F::cast_from(0.52945875e1_f64) * t10803 + F::cast_from(0.3529725e1_f64) * t10807 - t5852 + F::cast_from(0.20659e1_f64) * t7357 - F::cast_from(0.1549425e1_f64) * t9148 + F::cast_from(0.1549425e1_f64) * t10769 - F::cast_from(0.157790625e0_f64) * t10812 + F::cast_from(0.94674375e0_f64) * t10814 + F::cast_from(0.6311625e0_f64) * t10816 - t5859 + F::cast_from(0.104195e1_f64) * t7332 - F::cast_from(0.62517e0_f64) * t9185 - F::cast_from(0.62517e0_f64) * t9192 + F::cast_from(0.937755e0_f64) * t10823 + F::cast_from(0.312585e0_f64) * t10827;
    (t10834, t10841, t10842, t10859)
}
