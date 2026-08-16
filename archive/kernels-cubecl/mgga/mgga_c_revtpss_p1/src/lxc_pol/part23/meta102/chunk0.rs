//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 680/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk680<F: Float>(t158: F, t2609: F, t157: F, t37: F, t606: F, t750: F, t706: F, t186: F, t215: F, t685: F) -> (F, F, F, F, F) {
    let t2610 = t158 * t2609;
    let t2611 = t37 * t157;
    let t2615 = t750 * t606;
    let t2616 = t706 * t2615;
    let t2619 = t685 * t215 * t186;
    (t2610, t2611, t2615, t2616, t2619)
}
