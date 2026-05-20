//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2375/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2375<F: Float>(t10841: F, t10845: F, t10878: F, t2741: F, t2722: F, t853: F, t10726: F, t10786: F, t2661: F, t10943: F, t2663: F, t2645: F, t2662: F, t2749: F) -> (F, F, F, F, F, F) {
    let t40374 = t10845 * t10841;
    let t40376 = t2741 * t10878;
    let t40378 = t853 * t2722;
    let t40381 = t2661 * t10726 * t40378 * t10786;
    let t40385 = t2661 * t10726 * t2663 * t10943;
    let t40390 = t2661 * t2662 * t853 * t2645 * t2749;
    (t40374, t40376, t40378, t40381, t40385, t40390)
}
