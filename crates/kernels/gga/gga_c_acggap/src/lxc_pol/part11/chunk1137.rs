//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1137/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1137<F: Float>(t35608: F, t7433: F, t8787: F, t1165: F, t20433: F, t2068: F, t7351: F, t31362: F, t8956: F, t525: F, t839: F, t604: F, t7337: F) -> (F, F, F, F, F, F) {
    let t35609 = F::cast_from(0.20965394859736101378e-3_f64) * t35608;
    let t35610 = t7433 * t8787;
    let t35611 = F::cast_from(0.56606566121287473722e-2_f64) * t35610;
    let t35614 = t2068 * t1165 * t7351 * t20433;
    let t35616 = t31362 * t8956;
    let t35617 = F::cast_from(0.15724046144802076034e-2_f64) * t35616;
    let t35618 = t525 * t839;
    let t35621 = t7337 * t1165 * t604 * t35618;
    (t35609, t35611, t35614, t35617, t35618, t35621)
}
