//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1227/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1227<F: Float>(t13580: F, t24389: F, t4977: F, t27657: F, t3766: F, t108448: F, t213: F, t2360: F, t3886: F, t10915: F, t2347: F, t27617: F, t13521: F, t30683: F, t1614: F, t17836: F, t6018: F, t679: F) -> (F, F, F, F, F, F) {
    let t123403 = t13580 * t24389 * t4977;
    let t123408 = t3766 * t27657;
    let t123415 = t108448 * t213 * t2360 * t3886;
    let t123421 = t27617 * t10915 * t213 * t2347 * t3886;
    let t123424 = t30683 * t13521;
    let t123433 = t17836 * t6018 * t1614 * t679;
    (t123403, t123408, t123415, t123421, t123424, t123433)
}
