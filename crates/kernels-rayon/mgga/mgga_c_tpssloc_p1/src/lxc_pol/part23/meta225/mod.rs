//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta225(t1725: f64, t698: f64, t1174: f64, t5168: f64, t588: f64, t592: f64, t2528: f64, t5154: f64, t2535: f64, t118: f64, t1787: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15753, t15754, t15875, t15877, t15890, t15895, t15908) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk873(t1725, t698, t1174, t5168, t588, t592, t2528, t5154, t2535, t118, t1787);
    (t15753, t15754, t15875, t15877, t15890, t15895, t15908)
}
