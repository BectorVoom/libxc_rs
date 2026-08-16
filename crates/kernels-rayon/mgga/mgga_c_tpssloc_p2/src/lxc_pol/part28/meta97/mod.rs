//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk609;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk610;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta97(t2244: f64, t65: f64, t11: f64, t2219: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64) -> (f64, f64, f64, f64, f64) {
        let (t2245, t2248, t2249) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk609(t2244, t65, t11, t2219);
        let t2250 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk610(t25, t28, t2249, zeta_threshold);
        let t2251 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk611(t2250, t31);
    (t2245, t2248, t2249, t2250, t2251)
}
