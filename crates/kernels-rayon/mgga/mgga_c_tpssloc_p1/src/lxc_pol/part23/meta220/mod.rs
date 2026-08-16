//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta220(t1675: f64, t3331: f64, t15026: f64, t3623: f64, t1706: f64, t3428: f64, t135: f64, t457: f64, t11529: f64, t1709: f64, t1174: f64, t11588: f64, t1714: f64, t1716: f64, t698: f64, t1420: f64, t1887: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15207, t15245, t15265, t15281, t15299, t15300, t15338) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk867(t1675, t3331, t15026, t3623, t1706, t3428, t135, t457, t11529, t1709, t1174, t11588, t1714);
        let (t15363, t15364, t15376) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk868(t1716, t698, t1174, t1420, t1887, t337);
    (t15207, t15245, t15265, t15281, t15299, t15300, t15338, t15363, t15364, t15376)
}
