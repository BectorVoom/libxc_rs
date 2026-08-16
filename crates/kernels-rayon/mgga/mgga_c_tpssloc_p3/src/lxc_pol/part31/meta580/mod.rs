//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta580(t22716: f64, t7701: f64, t1834: f64, t212: f64, t22642: f64, t6890: f64, t81267: f64, t26215: f64, t81228: f64, t81326: f64, t6897: f64, t6907: f64, t90544: f64, t81284: f64, t26203: f64, t6883: f64, t7700: f64, t80645: f64, t214: f64, t5318: f64, t81311: f64, t26378: f64, t6914: f64, t1372: f64, t1799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90659, t90663, t90670, t90686, t90701) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818(t22716, t7701, t1834, t212, t22642, t6890, t81267, t26215, t81228, t81326, t6897, t6907, t90544);
        let (t90706, t90707, t90723, t90739, t90743, t90749, t90754) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1819(t81284, t26203, t6883, t6897, t7700, t80645, t214, t5318, t81311, t26378, t6914, t1372, t1799);
    (t90659, t90663, t90670, t90686, t90701, t90706, t90707, t90723, t90739, t90743, t90749, t90754)
}
