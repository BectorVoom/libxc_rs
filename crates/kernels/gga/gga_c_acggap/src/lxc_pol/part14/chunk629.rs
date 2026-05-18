//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 629/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk629<F: Float>(t5034: F, t1708: F, t75: F, t288: F, t1: F, t283: F, t2996: F, t2998: F, t3000: F, t5040: F, t5045: F, t1711: F, t224: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6020 = F::new(24.0) * t5034;
    let t6021 = t1708 * t75;
    let t6022 = t6021 * t288;
    let t6023 = F::new(0.5848223622634646207e0) * t6022;
    let t6024 = t1708 * t1;
    let t6025 = t6024 * t283;
    let t6026 = F::new(0.18311447306006545054e-3) * t6025;
    let t6027 = F::new(32.0) * t2996;
    let t6028 = F::new(20.0) * t2998;
    let t6029 = F::new(8.0) * t3000;
    let t6030 = F::new(0.34631718211362927517e2) * t5040;
    let t6031 = F::new(0.11696447245269292414e1) * t5045;
    let t6032 = t224 * t1711;
    (t6020, t6023, t6026, t6027, t6028, t6029, t6030, t6031, t6032)
}
