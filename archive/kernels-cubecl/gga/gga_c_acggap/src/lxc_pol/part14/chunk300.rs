//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 300/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk300<F: Float>(t1360: F, t1394: F, t150: F, t153: F, t94: F, t420: F, t495: F, t301: F, t1298: F, t402: F, t155: F, t400: F, t403: F, t519: F, t521: F) -> (F, F, F, F, F, F) {
    let t1396 = (t1360 + t1394) * t150;
    let t1402 = t153 * t94;
    let t1403 = t420 * t495;
    let t1404 = t1403 * t301;
    let t1407 = t402 * t1298;
    let t1410 = -t1396 * t155 - F::cast_from(12.0_f64) * t1402 * t1404 + F::cast_from(3.0_f64) * t1407 * t153 + F::cast_from(3.0_f64) * t400 * t521 + F::cast_from(3.0_f64) * t403 * t519;
    (t1396, t1402, t1403, t1404, t1407, t1410)
}
