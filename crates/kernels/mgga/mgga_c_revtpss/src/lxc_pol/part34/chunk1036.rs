//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1036/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1036<F: Float>(t1937: F, t29508: F, t7732: F, t7735: F, t21663: F, t38: F, t25132: F, t25137: F, t5819: F, t5825: F, t6968: F, t72: F, t1927: F, t7715: F, t7719: F, t5868: F, t76: F) -> (F, F, F, F, F, F, F, F) {
    let t29510 = 2.0 * t29508 * t1937;
    let t29512 = 4.0 * t7732 * t7735;
    let t29513 = t21663 * t38;
    let t29524 = 5.0 / 18.0 * t25132 * t5819 + 5.0 / 6.0 * t6968 * t5825 - t25137;
    let t29525 = t29524 * t72;
    let t29526 = t29525 * t1927;
    let t29529 = t7715 * t7719;
    let t29532 = t76 * t5868;
    (t29510, t29512, t29513, t29524, t29525, t29526, t29529, t29532)
}
