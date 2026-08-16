//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta617(t52367: f64, t3030: f64, t4940: f64, t3623: f64, t11712: f64, t11880: f64, t491: f64, t1734: f64, t6739: f64, t3609: f64, t3242: f64, t475: f64, t1174: f64, t44571: f64, t4724: f64, t11778: f64, t43791: f64, t1227: f64, t49850: f64, t4988: f64, t15568: f64, t3604: f64, t10401: f64, t15567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52368, t52434, t52435, t52479, t52480, t52485, t52548) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147(t52367, t3030, t4940, t3623, t11712, t11880, t491, t1734, t6739, t3609, t3242, t475);
        let (t52600, t52601, t52610, t52615, t52627) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2148(t1174, t44571, t4724, t11778, t43791, t1227, t49850, t4988, t15568, t3604, t10401, t15567);
    (t52368, t52434, t52435, t52479, t52480, t52485, t52548, t52600, t52601, t52610, t52615, t52627)
}
