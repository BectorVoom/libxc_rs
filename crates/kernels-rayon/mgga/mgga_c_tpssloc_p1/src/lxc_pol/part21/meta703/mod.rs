//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2533;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta703(t13542: f64, t13779: f64, t2986: f64, t13546: f64, t13555: f64, t13784: f64, t13528: f64, t1592: f64, t42891: f64, t973: f64, t13812: f64, t13822: f64, t13881: f64, t13886: f64, t10263: f64, t4506: f64, t3082: f64, t4622: f64, t1040: f64, t13941: f64, t10231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48384, t48387, t48390, t48394, t48397, t48402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2533(t13542, t13779, t2986, t13546, t13555, t13784, t13528, t1592, t42891, t973, t13812, t13822);
        let (t48407, t48417, t48421, t48430, t48432, t48441) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2534(t13822, t13881, t973, t13886, t10263, t4506, t3082, t4622, t1040, t13941, t10231, t13555);
    (t48384, t48387, t48390, t48394, t48397, t48402, t48407, t48417, t48421, t48430, t48432, t48441)
}
