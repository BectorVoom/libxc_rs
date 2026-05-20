//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2951/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2951<F: Float>(t2924: F, t78329: F, t934: F, t11385: F, t19255: F, t4631: F, t23466: F, t41499: F, t41502: F, t19330: F, t41361: F, t41908: F, t51978: F, t52397: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F) -> (F, F, F, F, F) {
    let t78332 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t78329 * t934;
    let t78335 = F::cast_from(0.1551780387578202009e4_f64) * t11385 * t19255 * t4631;
    let t78339 = F::cast_from(0.24955700379505800916e5_f64) * t41499 * t23466 * t41502 * t934;
    let t78342 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t19330 * t4631;
    let t78375 = F::cast_from(0.63419753086419753083e-2_f64) * t77499 - F::cast_from(0.17123333333333333333e-1_f64) * t77503 + F::cast_from(0.57077777777777777777e-2_f64) * t77505 - F::cast_from(0.2283111111111111111e-1_f64) * t77507 + F::cast_from(0.34246666666666666667e-1_f64) * t77509 - F::cast_from(0.34246666666666666666e-1_f64) * t63276 + F::cast_from(0.11415555555555555555e-1_f64) * t63278 + t41908 + F::cast_from(0.20547999999999999999e0_f64) * t77515 - F::cast_from(0.57077777777777777775e-1_f64) * t77518 - F::new(0.30822e0) * t77521 - t52397 + F::cast_from(0.5327259259259259259e-1_f64) * t51978 + F::cast_from(0.17757530864197530864e-1_f64) * t41361 - F::cast_from(0.34246666666666666665e-1_f64) * t77527 - F::cast_from(0.34246666666666666665e-1_f64) * t77531 + F::new(0.41096e0) * t77535 - F::new(0.30822e0) * t77539 + F::new(0.10274e0) * t77543 + F::new(0.10274e0) * t77547;
    (t78332, t78335, t78339, t78342, t78375)
}
