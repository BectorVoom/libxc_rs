//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 958/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk958<F: Float>(t7574: F, t8435: F, t2247: F, t196: F, t197: F, t7687: F, t4147: F, t7535: F, t2056: F, t27060: F, t29432: F, t32386: F, t32388: F, t32393: F, t32395: F, t32396: F, t32397: F, t32398: F, t32402: F, t32404: F, t7359: F, t7367: F, t7586: F, t7591: F) -> (F, F, F, F, F) {
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32822 = t7687 * t196 * t197;
    let t33183 = t4147 * t7535;
    let t33245 = -t2056 * t27060 - t2056 * t29432 - t7359 * t7591 - t7367 * t7586 - t32386 - t32388 - t32393 - t32395 - t32396 - t32397 - t32398 - t32402 - t32404;
    (t32805, t32806, t32822, t33183, t33245)
}
