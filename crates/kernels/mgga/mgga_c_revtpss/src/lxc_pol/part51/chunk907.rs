//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 907/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk907<F: Float>(t357: F, t988: F, t378: F, t42859: F, t1071: F, t11239: F, t7150: F, t3143: F, t36870: F, t11120: F, t3140: F, t25698: F, t1444: F, t543: F, t1419: F, t7063: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93437 = t357 * t988;
    let t93469 = t378 * t42859;
    let t93488 = t1071 * t11239;
    let t93962 = t7150 * t1071;
    let t93982 = t36870 * t3143;
    let t94014 = t3140 * t11120;
    let t94121 = t25698 * t378;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    (t93437, t93469, t93488, t93962, t93982, t94014, t94121, t94396, t94801)
}
