//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk651;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk652;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk653;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta100(t25: f64, t28: f64, t2249: f64, zeta_threshold: f64, t31: f64, t65: f64, t608: f64, t628: f64, t36: f64, t365: f64, t42: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t2250 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk651(t25, t28, t2249, zeta_threshold);
        let t2251 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk652(t2250, t31);
        let (t2252, t2255, t2261) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk653(t2251, t65, t608, t628, t36, t365);
        let (t2262, t2267) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk654(t2261, t42, sigma0);
    (t2250, t2251, t2252, t2255, t2261, t2262, t2267)
}
