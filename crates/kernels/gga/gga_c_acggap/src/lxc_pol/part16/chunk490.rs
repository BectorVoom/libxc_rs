//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 490/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk490<F: Float>(t2775: F, t690: F, t2671: F, t2674: F, t2677: F, t2679: F, t2683: F, t2685: F, t2687: F, t2690: F, t272: F, t264: F, t685: F, t75: F, t689: F, t83: F) -> (F, F, F, F, F, F) {
    let t2776 = t2775 * t690;
    let t2787 = -0.34523333333333333333e1 * t2671 + 0.23015555555555555556e1 * t2674 - 0.26851481481481481482e1 * t2677 - 0.93932222222222222223e0 * t2679 + 0.73355e-1 * t2683 - 0.14671e0 * t2685 - 0.17116166666666666667e0 * t2687 - 0.36793333333333333333e0 * t2690;
    let t2788 = t2787 * t272;
    let t2792 = 1.0 / t685 / t264;
    let t2793 = t75 * t2792;
    let t2795 = 1.0 / t689 / t83;
    (t2776, t2787, t2788, t2792, t2793, t2795)
}
