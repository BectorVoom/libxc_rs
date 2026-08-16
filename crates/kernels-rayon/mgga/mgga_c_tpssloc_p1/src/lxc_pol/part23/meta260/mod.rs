//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk922;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta260(t1358: f64, t6379: f64, t12211: f64, t6371: f64, t3726: f64, t6375: f64, t12385: f64, t6390: f64, t16288: f64, t1827: f64, t1340: f64, t19815: f64, t120: f64, t6387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19834, t19839, t19841, t19851, t19853, t19855) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk922(t1358, t6379, t12211, t6371, t3726, t6375, t12385, t6390, t16288, t1827, t1340, t19815);
        let t19871 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk923(t120, t6387);
    (t19834, t19839, t19841, t19851, t19853, t19855, t19871)
}
