//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1320/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1320<F: Float>(t94570: F, t94534: F, t94537: F, t94540: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F, t94569: F) -> F {
    let t94571 = F::cast_from(0.14450132032386466905e-2_f64) * t94570;
    let t94572 = -F::cast_from(0.85748036236139473943e-3_f64) * t94534 + F::cast_from(0.15246000842785598468e-4_f64) * t94537 - F::cast_from(0.1084295579938911763e-3_f64) * t94540 - F::cast_from(0.30492001685571196935e-3_f64) * t94542 - F::cast_from(0.13605355082800796533e0_f64) * t94546 + F::cast_from(0.24009450146119052704e-1_f64) * t94548 - F::cast_from(0.85748036236139473944e-4_f64) * t94552 - F::cast_from(0.45732285992607719437e-3_f64) * t94554 + F::cast_from(0.42874018118069736972e-4_f64) * t94557 - F::cast_from(0.12004725073059526352e0_f64) * t94559 + F::cast_from(0.15246000842785598468e-2_f64) * t94561 - F::cast_from(0.27107389498472794076e-4_f64) * t94565 - t94569 - t94571;
    t94572
}
