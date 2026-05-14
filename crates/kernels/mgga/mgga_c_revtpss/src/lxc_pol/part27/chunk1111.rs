//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1111/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1111<F: Float>(t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t94534: F, t94537: F, t94540: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F) -> (F,) {
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = 0.22589491248727328397e-6 * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = 0.14450132032386466905e-2 * t94570;
    let t94572 = -0.85748036236139473943e-3 * t94534 + 0.15246000842785598468e-4 * t94537 - 0.1084295579938911763e-3 * t94540 - 0.30492001685571196935e-3 * t94542 - 0.13605355082800796533e0 * t94546 + 0.24009450146119052704e-1 * t94548 - 0.85748036236139473944e-4 * t94552 - 0.45732285992607719437e-3 * t94554 + 0.42874018118069736972e-4 * t94557 - 0.12004725073059526352e0 * t94559 + 0.15246000842785598468e-2 * t94561 - 0.27107389498472794076e-4 * t94565 - t94569 - t94571;
    (t94572,)
}
