//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1354/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1354(t54566: f64, t51952: f64, t51954: f64, t51958: f64, t52562: f64, t54564: f64, t54572: f64, t54575: f64, t54581: f64, t54588: f64, t54593: f64, t54596: f64, t54605: f64, t54607: f64, t54613: f64) -> f64 {
    let t55863 = 7.0_f64 / 36.0_f64 * t54566;
    let t55877 = -t54564 / 48.0_f64 + t55863 + t54572 / 24.0_f64 - t54575 / 24.0_f64 + 7.0_f64 / 288.0_f64 * t52562 - t54581 / 16.0_f64 - t54588 / 384.0_f64 - t54593 / 192.0_f64 - t54596 / 24.0_f64 - 5.0_f64 / 192.0_f64 * t54605 - t54607 / 48.0_f64 + 7.0_f64 / 36.0_f64 * t51952 + 7.0_f64 / 576.0_f64 * t51954 - 7.0_f64 / 144.0_f64 * t51958 + t54613 / 24.0_f64;
    t55877
}
