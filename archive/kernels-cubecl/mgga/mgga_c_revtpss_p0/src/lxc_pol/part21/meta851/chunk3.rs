//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3200/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200<F: Float>(t17794: F, t372: F, t3584: F, t606: F, t1261: F, t17203: F, t3172: F, t43766: F, t44361: F, t12916: F, t17419: F, t5340: F) -> (F, F, F, F, F) {
    let t58960 = t372 * t17794;
    let t58969 = t3584 * t606;
    let t58975 = t1261 * t3172 * t17203;
    let t58983 = t44361 * t43766;
    let t58997 = t5340 * t12916 * t17419;
    (t58960, t58969, t58975, t58983, t58997)
}
