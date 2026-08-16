//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta1 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk8;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk9;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk10;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk11;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk12;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk13;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk14;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta1(t14: f64, t9: f64, t10: f64, t15: f64, t11: f64, t17: f64, t5: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19, t20, t21) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk8(t14, t9, t10, t15);
        let t24 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk9(t11, t17, t19, t21, t9);
        let t25 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk10(t5);
        let (t27, t28) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk11(t25, t5, zeta_threshold);
        let t31 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk12(t25, t28, t27, t5, zeta_threshold);
        let t32 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk13(t31);
        let t33 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk14(t32);
    (t19, t20, t21, t24, t25, t28, t31, t32, t33)
}
