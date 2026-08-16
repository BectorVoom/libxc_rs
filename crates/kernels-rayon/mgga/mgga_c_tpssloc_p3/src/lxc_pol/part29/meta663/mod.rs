//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta663(t26411: f64, t6914: f64, t12420: f64, t26331: f64, t5335: f64, t6976: f64, t1351: f64, t1992: f64, t5318: f64, t550: f64, t16036: f64, t22633: f64, t3807: f64, t12407: f64, t22704: f64, t22705: f64, t5345: f64, t54918: f64, t22690: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90760, t90763, t90770, t90774) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2204(t26411, t6914, t12420, t26331, t5335, t6976, t1351, t1992, t5318, t550, t16036, t22633, t3807);
        let (t90778, t90782, t90785, t90787) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2205(t12407, t22633, t5335, t6976, t22704, t22705, t5345, t1992, t54918, t550, t22690, t552);
    (t90760, t90763, t90770, t90774, t90778, t90782, t90785, t90787)
}
