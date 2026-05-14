//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 780/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk780<F: Float>(t1734: F, t467: F, t1941: F, t301: F, t1713: F, t157: F, t1772: F, t406: F, t1524: F, t524: F, t1410: F, t1753: F, t513: F, t1487: F, t1795: F, t1748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24753 = t1734 * t467;
    let t24794 = t1941 * t467;
    let t24811 = t1941 * t301;
    let t24893 = t1713 * t467;
    let t25706 = t1772 * t406 * t157;
    let t25727 = t1524 * t524 * t157;
    let t25732 = t1753 * t1410;
    let t25742 = t513 * t1410 * t157;
    let t25941 = t1734 * t406 * t157;
    let t26108 = t1487 * t524 * t157;
    let t26214 = t1795 * t406 * t157;
    let t26459 = t1748 * t1410;
    (t24753, t24794, t24811, t24893, t25706, t25727, t25732, t25742, t25941, t26108, t26214, t26459)
}
