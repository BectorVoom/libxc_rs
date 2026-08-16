//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2594;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta735(t15814: f64, t225: f64, t3030: f64, t4940: f64, t3623: f64, t1009: f64, t15425: f64, t1243: f64, t11712: f64, t11880: f64, t491: f64, t1734: f64, t6739: f64, t3609: f64, t1011: f64, t1212: f64, t11539: f64, t1174: f64, t14736: f64, t1227: f64, t13969: f64, t15544: f64, t15655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52386, t52434, t52435, t52446, t52447, t52479, t52480) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2594(t15814, t225, t3030, t4940, t3623, t1009, t15425, t1243, t11712, t11880, t491, t1734, t6739);
        let (t52485, t52568, t52575, t52580, t52583) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2595(t3609, t52434, t1011, t1212, t52446, t11539, t1174, t14736, t1227, t13969, t15544, t15655);
    (t52386, t52434, t52435, t52447, t52479, t52480, t52485, t52568, t52575, t52580, t52583)
}
