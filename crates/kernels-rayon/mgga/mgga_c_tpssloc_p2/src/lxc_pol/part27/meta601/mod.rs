//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2069;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta601(t1049: f64, t225: f64, t344: f64, t23384: f64, t23729: f64, t10189: f64, t1926: f64, t221: f64, t23337: f64, t10336: f64, t1920: f64, t1922: f64, t23391: f64, t6680: f64, t3173: f64, t3175: f64, t1921: f64, t1054: f64, t3206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82417, t82426, t82431) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2069(t1049, t225, t344, t23384, t23729, t10189, t1926, t221);
        let (t82432, t82436, t82437, t82442, t82457) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2070(t23337, t82431, t10336, t1920, t1922, t23391, t6680, t3173, t3175, t1921, t1054, t3206);
    (t82417, t82426, t82431, t82432, t82436, t82437, t82442, t82457)
}
