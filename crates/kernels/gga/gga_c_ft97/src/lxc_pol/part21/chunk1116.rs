//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1116/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1116<F: Float>(t1286: F, t29749: F, t376: F, t1307: F, t4621: F, t5731: F, t61025: F, t22914: F, t29578: F, t100050: F, t101922: F, t101932: F, t102065: F, t102066: F, t1564: F, t16060: F, t16150: F, t16155: F, t22907: F, t22908: F, t22935: F, t25558: F, t25615: F, t25616: F, t26119: F, t29615: F, t29616: F, t379: F, t4606: F, t5501: F, t91480: F, t93871: F, t94038: F) -> (F, F) {
    let t115210 = t1286 * t376 * t29749;
    let t115224 = t1307 * t4621;
    let t115229 = t61025 * t5731;
    let t115231 = t22914 * t29578;
    let t115248 = -t115210 / 3.0 + t101922 + 2.0 / 27.0 * t5501 * t93871 * t25616 * t16155 + 2.0 / 9.0 * t5501 * t25615 * t100050 * t16150 - 5.0 / 81.0 * t5501 * t102065 * t102066 * t16150 + t101932 - t5501 * t1564 * t115224 * t379 / 18.0 + 4.0 * t115229 + t115231 / 27.0 - t25558 * t26119 / 9.0 + 2.0 / 9.0 * t22935 * t29616 + 2.0 / 9.0 * t5501 * t94038 * t29615 + 2.0 / 9.0 * t5501 * t22907 * t91480 * t4606 + 2.0 / 9.0 * t5501 * t22907 * t22908 * t16060;
    (t115229, t115248)
}
