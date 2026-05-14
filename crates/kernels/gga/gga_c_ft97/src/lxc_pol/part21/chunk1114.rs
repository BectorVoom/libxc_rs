//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1114/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1114<F: Float>(t6454: F, t984: F, t25587: F, t6414: F, t1286: F, t29571: F, t376: F, t100085: F, t100099: F, t101678: F, t102071: F, t1564: F, t16011: F, t16150: F, t16155: F, t22907: F, t25584: F, t25601: F, t25609: F, t25610: F, t25615: F, t25616: F, t28: F, t29745: F, t379: F, t432: F, t4621: F, t5495: F, t5501: F, t5507: F, t6421: F, t6457: F) -> (F,) {
    let t115144 = t6454 * t984;
    let t115149 = t6414 * t25587;
    let t115156 = t1286 * t376 * t29571;
    let t115169 = -t1286 * t28 * t5507 * t4621 * t432 / 3.0 - t100085 + t25584 * t6457 / 3.0 - 2.0 / 3.0 * t5495 * t29745 - 2.0 / 3.0 * t1286 * t28 * t101678 * t6421 - t100099 - t5501 * t1564 * t115144 * t379 / 9.0 + 2.0 / 9.0 * t115149 - t5501 * t25615 * t25616 * t16011 / 27.0 - t115156 / 18.0 - 2.0 / 9.0 * t5501 * t22907 * t25610 * t16155 - t5501 * t25609 * t25616 * t16150 / 3.0 + 2.0 / 9.0 * t5501 * t102071 * t25601;
    (t115169,)
}
