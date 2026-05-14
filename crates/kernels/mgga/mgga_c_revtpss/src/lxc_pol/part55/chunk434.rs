//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 434/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk434<F: Float>(t162: F, t2608: F, t158: F, t157: F, t37: F, t606: F, t750: F, t706: F, t186: F, t215: F, t685: F, t755: F, t72: F, t752: F, t757: F, t2492: F, t2596: F, t745: F) -> (F, F, F, F, F, F, F) {
    let t2609 = t162 * t2608;
    let t2610 = t158 * t2609;
    let t2611 = t37 * t157;
    let t2615 = t750 * t606;
    let t2616 = t706 * t2615;
    let t2619 = t685 * t215 * t186;
    let t2621 = 0.24415263074675393405e-3 * t755 * t2619;
    let t2622 = t752 * t72;
    let t2623 = t2622 * t757;
    let t2626 = t2596 * t2492 * t745;
    (t2610, t2611, t2616, t2619, t2621, t2623, t2626)
}
