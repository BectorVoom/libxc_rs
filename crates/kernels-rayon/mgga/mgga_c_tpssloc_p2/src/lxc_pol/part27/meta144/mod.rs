//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta144 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk814;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk815;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk816;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk817;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk818;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk819;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta144(t3158: f64, t339: f64, t964: f64, t995: f64, t1000: f64, t1020: f64, t1025: f64, t1046: f64, t2955: f64, t2960: f64, t3109: f64, t3114: f64, t3117: f64, t3123: f64, t3130: f64, t3134: f64, t3140: f64, t3143: f64, t3148: f64, t3153: f64, t3156: f64, t350: f64, t973: f64, t3106: f64, t349: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1065: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3160, t3163, t3165) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk813(t3158, t339, t964, t995, t1000, t1020, t1025, t1046, t2955, t2960, t3109, t3114, t3117, t3123, t3130, t3134, t3140, t3143, t3148, t3153, t3156, t350, t973);
        let t3166 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk814(t3106, t3165);
        let (t3167, t3169) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk815(t3166, t349, t1050, t225);
        let (t3173, t3174) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk816(t1053, t386, t68);
        let t3175 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk817(t1065);
        let t3176 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk818(t3174, t3175);
        let t3180 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk819(t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk820(t3032, t3127, t3031);
    (t3160, t3163, t3166, t3167, t3169, t3173, t3174, t3175, t3176, t3180, t3185, t3186)
}
