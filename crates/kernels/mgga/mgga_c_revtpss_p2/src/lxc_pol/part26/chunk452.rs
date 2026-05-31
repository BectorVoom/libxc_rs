//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 452/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk452<F: Float>(t162: F, t2608: F, t158: F, t157: F, t37: F, t190: F, t2251: F, t606: F, t750: F, t706: F, t186: F, t215: F, t685: F) -> (F, F, F, F, F, F, F, F) {
    let t2609 = t162 * t2608;
    let t2610 = t158 * t2609;
    let t2611 = t37 * t157;
    let t2612 = t190 * t2251;
    let t2614 = F::cast_from(12.0_f64) * t2611 * t2612;
    let t2615 = t750 * t606;
    let t2616 = t706 * t2615;
    let t2617 = F::cast_from(8.0_f64) * t2616;
    let t2619 = t685 * t215 * t186;
    (t2609, t2610, t2611, t2612, t2614, t2615, t2617, t2619)
}
