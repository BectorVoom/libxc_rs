//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk853;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk854;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk855;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta155(t3375: f64, t440: f64, t1155: f64, t1156: f64, t3236: f64, t3293: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3288: f64, t3290: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64, t1146: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3376, t3377) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk853(t3375, t440, t1155);
        let (t3378, t3383, t3390, t3395) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk854(t1156, t3377, t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
        let (t3396, t3399, t3400) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk855(t1156, t3395, t1146);
        let (t3401, t3402, t3403) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk856(t3400, t440, t448);
    (t3376, t3377, t3378, t3383, t3390, t3395, t3396, t3399, t3400, t3401, t3402, t3403)
}
