//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta125 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk730;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk731;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk732;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk733;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk734;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk735;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta125(t3158: f64, t339: f64, t964: f64, t995: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64, t1932: f64, t3131: f64, t1014: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3160, t3163, t3169) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk730(t3158, t339, t964, t995, t1050, t225);
        let (t3173, t3174) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk731(t1053, t386, t68);
        let t3180 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk732(t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk733(t3032, t3127, t3031);
        let t3188 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk734(t1932, t3131);
        let (t3199, t3200) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk735(t1014, t3032, t3031);
        let t3201 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk736(t1932, t360);
    (t3160, t3163, t3169, t3173, t3174, t3180, t3185, t3186, t3188, t3199, t3200, t3201)
}
