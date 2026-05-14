//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1062/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1062<F: Float>(t13807: F, t13916: F, t13920: F, t14791: F, t2417: F, t353: F, t859: F, t2242: F, t4055: F, t2306: F, t332: F, t2382: F, t2419: F, t4387: F, t892: F, t13928: F, t4386: F) -> (F, F, F, F, F, F, F, F) {
    let t51563 = t13807 * t13916;
    let t51564 = t51563 * t13920;
    let t51569 = t859 * t353 * t14791 * t2417;
    let t51572 = t2242 * t4055;
    let t51580 = t2306 * t332;
    let t51581 = t2382 * t51580;
    let t51584 = t859 * t2419;
    let t51588 = t859 * t892 * t4387;
    let t51592 = t4386 * t892 * t13928;
    (t51563, t51564, t51569, t51572, t51581, t51584, t51588, t51592)
}
