//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1033/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1033<F: Float>(t39840: F, t39841: F, t6064: F, t2608: F, t37470: F, t574: F, t19839: F, t20: F, t3293: F, t2124: F, t24762: F, t10810: F, t1592: F, t8160: F, t2196: F, t7615: F) -> (F, F, F, F, F) {
    let t39843 = t39840 * t39841 * t6064;
    let t39846 = t574 * t37470 * t2608;
    let t39849 = t3293 * t19839 * t20;
    let t39851 = t39849 * t2124 * t24762;
    let t39854 = t1592 * t10810 * t8160;
    let t39855 = 0.69345773920434148506e0 * t39854;
    let t39857 = t2196 * t10810 * t7615;
    (t39843, t39846, t39851, t39855, t39857)
}
