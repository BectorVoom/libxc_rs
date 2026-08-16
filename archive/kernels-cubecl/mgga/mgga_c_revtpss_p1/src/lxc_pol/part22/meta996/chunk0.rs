//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3385/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385<F: Float>(t19256: F, t41583: F, t11385: F, t19255: F, t2918: F, t2875: F, t41499: F, t41502: F, t6109: F, t4707: F, t972: F, t4711: F, t52238: F) -> (F, F, F, F, F) {
    let t63589 = F::cast_from(0.1034520258385468006e4_f64) * t41583 * t19256;
    let t63592 = F::cast_from(0.51726012919273400301e3_f64) * t11385 * t19255 * t2918;
    let t63596 = F::cast_from(0.24955700379505800916e5_f64) * t41499 * t6109 * t41502 * t2875;
    let t63597 = t972 * t4707;
    let t63600 = F::cast_from(0.4155806185363551302e3_f64) * t52238 * t4711 * t63597;
    (t63589, t63592, t63596, t63597, t63600)
}
