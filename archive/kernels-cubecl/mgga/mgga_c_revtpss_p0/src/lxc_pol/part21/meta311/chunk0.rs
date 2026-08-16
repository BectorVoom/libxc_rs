//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1577/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1577<F: Float>(t10832: F, t2484: F, t2749: F, t836: F, t853: F, t2662: F, t2661: F, t221: F, t2485: F, t2646: F, t2482: F, t596: F, t823: F) -> (F, F, F, F, F, F) {
    let t10833 = t2484 * t10832;
    let t10836 = t853 * t836 * t2749;
    let t10837 = t2662 * t10836;
    let t10838 = t2661 * t10837;
    let t10841 = t2485 * t221 * t2646;
    let t10842 = t2484 * t10841;
    let t10845 = t2482 * t823 * t596;
    (t10833, t10837, t10838, t10841, t10842, t10845)
}
