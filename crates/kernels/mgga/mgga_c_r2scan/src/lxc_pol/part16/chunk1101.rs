//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1101/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1101<F: Float>(t19839: F, t20: F, t3293: F, t10810: F, t1592: F, t8160: F, t2196: F, t7615: F, t2147: F, t2608: F, t38168: F, t10855: F, t128: F, t512: F) -> (F, F, F, F, F) {
    let t39849 = t3293 * t19839 * t20;
    let t39854 = t1592 * t10810 * t8160;
    let t39855 = F::cast_from(0.69345773920434148506e0_f64) * t39854;
    let t39857 = t2196 * t10810 * t7615;
    let t39858 = F::cast_from(0.27738309568173659402e1_f64) * t39857;
    let t39882 = t2147 * t38168 * t2608;
    let t39885 = t512 * t10855 * t128;
    (t39849, t39855, t39858, t39882, t39885)
}
