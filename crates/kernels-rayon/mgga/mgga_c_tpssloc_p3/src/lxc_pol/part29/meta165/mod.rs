//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta165 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk878;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk879;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk880;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk881;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk882;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk883;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk884;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta165(t3584: f64, t61: f64, t248: f64, t3243: f64, t1174: f64, t1213: f64, t1218: f64, t1227: f64, t1232: f64, t3490: f64, t3496: f64, t3506: f64, t3511: f64, t3515: f64, t3518: f64, t3524: f64, t3527: f64, t3531: f64, t3536: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3552: f64, t3557: f64, t3562: f64, t3567: f64, t3573: f64, t3577: f64, t3580: f64, t488: f64, t466: f64, t1236: f64, t225: f64, t1239: f64, t496: f64, t68: f64, t1251: f64, t1243: f64, t3534: f64, t3032: f64, t3502: f64, t3499: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3585, t3587) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk878(t3584, t61, t248, t3243);
        let t3590 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk879(t1174, t1213, t1218, t1227, t1232, t3490, t3496, t3506, t3511, t3515, t3518, t3524, t3527, t3531, t3536, t3542, t3543, t3547, t3549, t3552, t3557, t3562, t3567, t3573, t3577, t3580, t3587, t488);
        let (t3591, t3593) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk880(t3590, t466, t1236, t225);
        let (t3597, t3598) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk881(t1239, t496, t68);
        let t3599 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk882(t1251);
        let t3600 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk883(t3598, t3599);
        let t3604 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk884(t1243, t3534);
        let (t3609, t3610) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk885(t3032, t3502, t3499);
    (t3585, t3587, t3590, t3591, t3593, t3597, t3598, t3599, t3600, t3604, t3609, t3610)
}
