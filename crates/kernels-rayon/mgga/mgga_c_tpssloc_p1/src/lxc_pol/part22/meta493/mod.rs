//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1919;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta493(t17800: f64, t4514: f64, t17794: f64, t4531: f64, t10339: f64, t13896: f64, t17764: f64, t17770: f64, t17827: f64, t17850: f64, t21410: f64, t21413: f64, t21416: f64, t2986: f64, t973: f64, t17817: f64, t17804: f64, t10295: f64, t13642: f64, t17286: f64, t17288: f64, t17290: f64, t21120: f64, t21132: f64, t21136: f64, t21140: f64, t21161: f64, t21168: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t21419, t21422, t21429) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1919(t17800, t4514, t17794, t4531, t10339, t13896, t17764, t17770, t17827, t17850, t21410, t21413, t21416, t2986, t973);
        let (t21430, t21433, t21444) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1920(t17817, t4531, t17804, t4514, t10295, t13642, t17286, t17288, t17290, t21120, t21132, t21136, t21140, t21161, t21168);
    (t21419, t21422, t21429, t21430, t21433, t21444)
}
