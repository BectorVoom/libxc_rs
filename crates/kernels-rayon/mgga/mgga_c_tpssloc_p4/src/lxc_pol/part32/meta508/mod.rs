//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta508(t1827: f64, t22756: f64, t5289: f64, t6945: f64, t5310: f64, t6952: f64, t1824: f64, t236: f64, t22705: f64, t550: f64, t22852: f64, t2002: f64, t5230: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t26236, t26238, t26240, t26245, t26246, t26248) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1834(t1827, t22756, t5289, t6945, t5310, t6952, t1824, t236, t22705, t550, t22852, t2002, t5230);
    (t26236, t26238, t26240, t26245, t26246, t26248)
}
