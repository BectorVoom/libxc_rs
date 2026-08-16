//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta250(t761: f64, t9919: f64, t2531: f64, t2535: f64, t2427: f64, t2430: f64, t185: f64, t9258: f64, t707: f64, t32: f64, t717: f64, t2659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9921, t9922, t9923, t9924, t9925, t9926, t9928, t9929, t9931) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1373(t761, t9919, t2531, t2535, t2427, t2430, t185, t9258, t707, t32, t717, t2659);
    (t9921, t9922, t9923, t9924, t9925, t9926, t9928, t9929, t9931)
}
