//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1274/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1274<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1450: F, t4135: F, t177: F, t3850: F, t762: F, t749: F, t1331: F, t3857: F) -> (F, F, F, F, F, F) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9547 = t4135 * t1450;
    let t9551 = t3850 * t177;
    let t9552 = t9551 * t762;
    let t9554 = t3850 * t749;
    let t9555 = t512 * t9554;
    let t9559 = t3857 * t1331;
    (t9544, t9546, t9547, t9552, t9555, t9559)
}
