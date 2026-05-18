//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 692/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk692<F: Float>(t286: F, t818: F, t442: F, t7592: F, t2394: F, t825: F, t2723: F, t918: F, t1018: F, t2520: F, t1044: F, t125: F) -> (F, F, F, F, F, F) {
    let t7593 = t818 * t286;
    let t7595 = t7592 * t7593 * t442;
    let t7624 = t2394 * t825;
    let t7626 = t918 * t2723;
    let t7675 = t2520 * t1018;
    let t7676 = t1044 * t125;
    (t7593, t7595, t7624, t7626, t7675, t7676)
}
