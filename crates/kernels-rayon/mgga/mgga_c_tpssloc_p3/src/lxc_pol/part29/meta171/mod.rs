//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk905;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk906;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta171(t210: f64, t214: f64, t3734: f64, t1314: f64, t792: f64, t118: f64, t1307: f64, t794: f64, t3719: f64, t116: f64, t534: f64, t212: f64, t2586: f64, t1315: f64, t3725: f64, t3727: f64, t3731: f64, t3733: f64, t562: f64, t1323: f64, t1372: f64, t1324: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3736, t3739, t3741, t3742, t3745, t3748, t3749) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk905(t210, t214, t3734, t1314, t792, t118, t1307, t794, t3719, t116, t534, t212);
        let (t3751, t3752) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk906(t2586, t3749, t1315, t3725, t3727, t3731, t3733, t3736, t3742, t3745);
        let (t3753, t3755, t3758) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk907(t3752, t562, t1323, t1372, t1324, t225);
    (t3736, t3739, t3741, t3742, t3745, t3748, t3749, t3751, t3752, t3753, t3755, t3758)
}
