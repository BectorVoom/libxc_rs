//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 970/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk970<F: Float>(t31643: F, t527: F, t2001: F, t5255: F, t5003: F, t1418: F, t7605: F, t5260: F, t4547: F, t1347: F, t1980: F, t35383: F, t7458: F, t31773: F, t8634: F, t13299: F, t33944: F, t33945: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35664 = t31643 * t527;
    let t35668 = t2001 * t5255;
    let t35670 = t2001 * t5003;
    let t35672 = t7605 * t1418;
    let t35674 = t2001 * t5260;
    let t35676 = t2001 * t4547;
    let t35678 = t7605 * t1347;
    let t35682 = t1980 * t7458 * t35383;
    let t35685 = t31773 * t8634;
    let t35691 = t33944 * t13299 * t33945;
    (t35664, t35668, t35670, t35672, t35674, t35676, t35678, t35682, t35685, t35691)
}
