//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta521(t22152: f64, t22202: f64, t22267: f64, t22325: f64, t466: f64, t1720: f64, t6238: f64, t1751: f64, t6150: f64, t1734: f64, t1246: f64, t22298: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22327, t22328, t22334, t22337, t22340, t22341, t22348) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1987(t22152, t22202, t22267, t22325, t466, t1720, t6238, t1751, t6150, t1734, t1246, t22298, t491);
    (t22327, t22328, t22334, t22337, t22340, t22341, t22348)
}
