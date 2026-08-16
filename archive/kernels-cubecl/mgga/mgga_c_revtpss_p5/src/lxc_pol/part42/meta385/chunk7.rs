//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1277/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1277<F: Float>(t1678: F, t3316: F, t342: F, t6299: F, t73: F, t4976: F, t1082: F, t19414: F, t1045: F, t999: F, t6271: F, t3117: F) -> (F, F, F, F, F) {
    let t19607 = t3316 * t1678;
    let t19608 = t342 * t19607;
    let t19611 = t6299 * t73;
    let t19612 = t19611 * t4976;
    let t19617 = t1082 * t19414;
    let t19620 = t1045 * t999;
    let t19621 = t6271 * t19620;
    let t19622 = t3117 * t19621;
    (t19608, t19611, t19612, t19617, t19622)
}
