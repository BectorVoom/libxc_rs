//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1245;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta339(t13969: f64, t4599: f64, t3039: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t4595: f64, t3130: f64, t3048: f64, t4571: f64, t3109: f64, t4630: f64, t3108: f64, t4640: f64, t1611: f64, t3047: f64, t3103: f64, t4641: f64, t1040: f64, t4616: f64, t1612: f64, t3082: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13972, t13995, t14000, t14027, t14049) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1245(t13969, t4599, t3039, t3069, t4669, t10231, t4338, t973, t4595, t3130, t3048, t4571);
        let (t14059, t14077, t14080, t14084, t14085, t14117) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1246(t3109, t4630, t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1612, t3082);
    (t13972, t13995, t14000, t14027, t14049, t14059, t14077, t14080, t14084, t14085, t14117)
}
