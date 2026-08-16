//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1757/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1757<F: Float>(t90437: F, t90449: F, t1139: F, t43821: F, t90422: F, t43814: F, t43817: F, t89824: F, t89832: F, t90402: F, t90405: F, t90408: F, t90411: F, t90414: F, t90417: F, t90420: F, t90423: F) -> (F, F, F, F) {
    let t90450 = t90437 + t90449;
    let t90451 = t1139 * t90450;
    let t90453 = t43821 * t90422;
    let t90456 = -F::cast_from(0.82785e-1_f64) * t90402 + F::cast_from(0.49671e0_f64) * t90405 - F::cast_from(0.99342e0_f64) * t90408 + F::cast_from(0.198684e1_f64) * t90411 + F::cast_from(0.82785e-1_f64) * t90414 - F::cast_from(0.8585111111111111111e-1_f64) * t90417 - F::cast_from(0.3883875e1_f64) * t90420 + F::cast_from(0.6189328125e-1_f64) * t90423 - F::cast_from(0.89459259259259259259e0_f64) * t89832 + t43814 + t43817 + F::cast_from(0.16504875e0_f64) * t90451 - F::cast_from(0.485484375e1_f64) * t90453 + F::cast_from(0.20128333333333333334e1_f64) * t89824;
    (t90450, t90451, t90453, t90456)
}
