//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2039;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta544(t2223: f64, t3826: f64, t11985: f64, t25: f64, t514: f64, t11998: f64, t28: f64, t517: f64, t32253: f64, t59: f64, t154: f64, t541: f64, t12289: f64, t1336: f64, t835: f64, t12364: f64, t3777: f64, t1314: f64, t9569: f64, t1329: f64, t2559: f64, t3732: f64, t12214: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39857, t39861, t39877, t39933, t39934, t39936) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2039(t2223, t3826, t11985, t25, t514, t11998, t28, t517, t32253, t59, t154, t541);
        let (t39944, t39947, t40005, t40006, t40018, t40021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2040(t12289, t1336, t835, t12364, t3777, t1314, t9569, t1329, t2559, t3732, t12214, t782);
    (t39857, t39861, t39877, t39933, t39934, t39936, t39944, t39947, t40005, t40006, t40018, t40021)
}
