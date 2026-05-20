//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2039/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2039<F: Float>(t25875: F, t94394: F, t94398: F, t46361: F, t545: F, t25880: F, t9685: F, t25895: F, t25900: F, t94596: F, t25904: F, t1032: F, t9656: F) -> (F, F, F, F, F, F, F, F) {
    let t94649 = t25875 * t94394;
    let t94650 = t94649 * t94398;
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    let t94662 = t25895 * t94661;
    let t94664 = t94596 * t25900;
    let t94665 = t25904 * t94664;
    let t94667 = t1032 * t9656;
    (t94649, t94650, t94656, t94661, t94662, t94664, t94665, t94667)
}
