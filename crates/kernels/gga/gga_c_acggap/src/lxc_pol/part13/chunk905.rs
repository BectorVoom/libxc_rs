//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 905/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk905<F: Float>(t30811: F, t3363: F, t3367: F, t1530: F, t7432: F, t7415: F, t30154: F, t30781: F, t7586: F, t30153: F, t3360: F, t1101: F, t1992: F, t7842: F) -> (F, F, F, F, F, F, F) {
    let t30812 = t30811 * t3363;
    let t30814 = t30811 * t3367;
    let t30817 = t1530 * t7432;
    let t30818 = t30817 * t7415;
    let t30821 = t30154 * t7586 * t30781;
    let t30827 = t3360 * t30153;
    let t30830 = t30827 * t7842 * t1992 * t1101;
    (t30812, t30814, t30817, t30818, t30821, t30827, t30830)
}
