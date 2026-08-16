//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1200;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta291(t2563: f64, t2610: f64, t225: f64, t2592: f64, t2710: f64, t814: f64, t252: f64, t2678: f64, t856: f64, t68: f64, t2745: f64, t870: f64, t261: f64, t2751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10038, t10049, t10076, t10097, t10108, t10109, t10110, t10126) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1200(t2563, t2610, t225, t2592, t2710, t814, t252, t2678, t856, t68, t2745, t870);
        let t10143 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1201(t261, t2751);
    (t10038, t10049, t10076, t10097, t10108, t10109, t10110, t10126, t10143)
}
