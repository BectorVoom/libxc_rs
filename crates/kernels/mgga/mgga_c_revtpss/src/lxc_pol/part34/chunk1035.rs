//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1035/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1035<F: Float>(t27955: F, t1450: F, t6816: F, t7237: F, t2014: F, t6836: F, t25864: F, t1843: F, t7741: F, t651: F, t196: F, t197: F, t6773: F, t2035: F, t5920: F, t94: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28885 = 7.0 / 72.0 * t27955;
    let t29494 = t1450 * t6816;
    let t29495 = t7237 * t29494;
    let t29497 = 3.0 * t2014 * t29495;
    let t29498 = t1450 * t6836;
    let t29499 = t25864 * t29498;
    let t29501 = 6.0 * t2014 * t29499;
    let t29502 = t1843 * t7741;
    let t29504 = 4.0 * t651 * t29502;
    let t29506 = t6773 * t196 * t197;
    let t29507 = t29506 * t2035;
    let t29508 = t94 * t5920;
    (t28885, t29494, t29495, t29497, t29498, t29499, t29501, t29502, t29504, t29506, t29507, t29508)
}
