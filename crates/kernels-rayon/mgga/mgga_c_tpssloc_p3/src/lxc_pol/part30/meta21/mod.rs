//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk155;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk156;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk157;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk158;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk159;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk160;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta21(t120: f64, t61: f64, t283: f64, t374: f64, t339: f64, t350: f64, t370: f64, t349: f64, t362: f64, t68: f64, t353: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t375, t376) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk155(t120, t61, t283);
        let t378 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk156(t374, t375, t376);
        let t381 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk157(t339, t350, t370, t378);
        let (t382, t383) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk158(t349, t381, t362, t68);
        let t384 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk159(t381, t383);
        let (t386, t387) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk160(t353, t384);
        let t388 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk161(t254, t387);
    (t375, t376, t378, t381, t382, t383, t384, t386, t387, t388)
}
