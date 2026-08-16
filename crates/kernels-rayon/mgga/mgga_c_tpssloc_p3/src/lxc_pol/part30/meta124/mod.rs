//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk725;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk726;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk727;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk728;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta124(t1004: f64, t1040: f64, t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64, t360: f64, t135: f64, t999: f64, t973: f64, t2770: f64, t2978: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64, t221: f64, t2965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3117, t3127) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk725(t1004, t1040, t1013, t361);
        let t3128 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk726(t3127, t363);
        let (t3129, t3130) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk727(t3037, t3128, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk728(t360);
        let (t3139, t3140, t3146, t3151, t3156, t3158) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk729(t135, t999, t973, t2770, t2978, t2775, t976, t1005, t1036, t221, t2965);
    (t3117, t3127, t3128, t3129, t3130, t3131, t3139, t3140, t3146, t3151, t3156, t3158)
}
