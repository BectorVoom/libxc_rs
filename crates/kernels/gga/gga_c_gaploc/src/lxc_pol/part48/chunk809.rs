//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 809/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk809<F: Float>(t13621: F, t5782: F, t36390: F, t787: F, t9824: F, t10914: F, t2365: F, t35446: F, t1: F, t44766: F, t13555: F, t4614: F, t833: F, t10811: F, t10978: F, t2028: F, t44070: F, t44084: F, t44088: F, t44114: F, t44117: F, t44120: F, t45877: F, t45882: F, t45885: F, t45887: F, t45888: F, t45892: F, t45894: F, t45898: F) -> (F,) {
    let t45900 = 0.69017266717057349418e1 * t5782 * t13621;
    let t45902 = t787 * t36390 * t9824;
    let t45903 = 0.14896037479937677779e-1 * t45902;
    let t45905 = t10914 * t2365 * t35446;
    let t45906 = 0.89376224879626066674e-1 * t45905;
    let t45908 = t787 * t44766 * t1;
    let t45913 = 0.15337170381568299871e2 * t833 * t4614 * t13555;
    let t45915 = 0.85801175884441024006e1 * t10811 * t10978;
    let t45919 = t45877 - 0.11916829983950142223e0 * t44070 - 0.63904876589867916128e-1 * t44084 - 0.63904876589867916128e-1 * t44088 + t45882 + t45885 + t45887 + 0.89376224879626066676e-1 * t45888 - t45892 - t45894 - t45898 - t45900 + t45903 - t45906 - 0.39722766613167140743e-1 * t45908 * t2028 + t45913 + t45915 - 0.17875244975925213335e0 * t44114 - 0.63904876589867916128e-1 * t44117 + 0.1022478025437886658e1 * t44120;
    (t45919,)
}
