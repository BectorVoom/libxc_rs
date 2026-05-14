//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 988/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk988<F: Float>(t19839: F, t822: F, t833: F, t2397: F, t6745: F, t2242: F, t2355: F, t6810: F, t8801: F, t2074: F, t2182: F, t19819: F, t19821: F, t19824: F, t19829: F, t19836: F, t2376: F, t2395: F, t2408: F, t2409: F, t2417: F, t3067: F, t3207: F, t4385: F, t6127: F, t6449: F, t6723: F, t810: F, t831: F, t9241: F, t9296: F) -> (F, F) {
    let t19841 = t822 * t19839 * t833;
    let t19843 = t6745 * t2397;
    let t19845 = t2242 * t2355;
    let t19857 = t8801 * t6810;
    let t19859 = t2074 * t2182;
    let t19869 = -35.0 / 72.0 * t19819 - 7.0 / 24.0 * t19821 + t4385 * t19824 / 4.0 - 3.0 / 8.0 * t4385 * t19829 + 3.0 / 4.0 * t3207 * t2409 * t2395 * t6449 - 7.0 / 12.0 * t19836 - 455.0 / 324.0 * t19841 - 7.0 / 24.0 * t19843 - 35.0 / 72.0 * t19845 + t2408 * t2409 * t9296 * t6127 * t810 / 2.0 + t2408 * t2409 * t2376 * t6723 * t810 / 12.0 - 7.0 / 4.0 * t19857 - 3.0 / 2.0 * t9241 * t2409 * t831 * t19859 + 3.0 / 4.0 * t3207 * t2409 * t3067 * t2182 * t2417;
    (t19859, t19869)
}
