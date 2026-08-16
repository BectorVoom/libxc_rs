//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1913;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta606(t1351: f64, t1992: f64, t5318: f64, t550: f64, t6976: f64, t16036: f64, t22633: f64, t3807: f64, t12407: f64, t5335: f64, t22704: f64, t22705: f64, t5345: f64, t54918: f64, t22690: f64, t552: f64, t26447: f64, t90607: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90770, t90774, t90778, t90781) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1913(t1351, t1992, t5318, t550, t6976, t16036, t22633, t3807, t12407, t5335, t22704, t22705, t5345);
        let (t90785, t90787, t90789, t90791, t90794) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1914(t1992, t54918, t550, t6976, t22690, t552, t26447, t90607, t22751, t26397, t22892, t22893, t26396);
    (t90770, t90774, t90778, t90781, t90785, t90787, t90789, t90791, t90794)
}
