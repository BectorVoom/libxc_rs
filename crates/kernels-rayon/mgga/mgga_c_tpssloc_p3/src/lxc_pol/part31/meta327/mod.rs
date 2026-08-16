//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1222;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta327(t12858: f64, t763: f64, t1472: f64, t2517: f64, t4303: f64, t870: f64, t2430: f64, t4205: f64, t1409: f64, t750: f64, t607: f64, t4194: f64, t3966: f64, t751: f64, t707: f64, t157: f64, t9897: f64, t2371: f64, t4199: f64, t1484: f64, t212: f64, t9523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12860, t12861, t12895, t12922, t12926) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1222(t12858, t763, t1472, t2517, t4303, t870, t2430, t4205, t1409, t750, t607, t4194);
        let (t12934, t12939, t12943, t12946, t12984, t12985) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1223(t3966, t751, t707, t157, t9897, t2371, t4199, t1409, t2517, t1484, t212, t9523);
    (t12860, t12861, t12895, t12922, t12926, t12934, t12939, t12943, t12946, t12984, t12985)
}
