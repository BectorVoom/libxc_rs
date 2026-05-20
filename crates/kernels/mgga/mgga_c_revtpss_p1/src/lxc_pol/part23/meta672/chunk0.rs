//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2408/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2408<F: Float>(t3046: F, t4980: F, t12046: F, t989: F, t1035: F, t42859: F, t342: F, t12166: F, t16409: F, t994: F, t3057: F, t11223: F, t3286: F) -> (F, F, F, F, F, F, F, F) {
    let t43360 = t3046 * t4980;
    let t43384 = t989 * t12046;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43420 = t989 * t12166;
    let t43432 = t994 * t16409;
    let t43438 = t3057 * t4980;
    let t43443 = t11223 * t3286;
    (t43360, t43384, t43400, t43401, t43420, t43432, t43438, t43443)
}
