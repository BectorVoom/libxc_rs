//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 777/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk777(t3637: f64, t8675: f64, t358: f64, t3653: f64, t363: f64, t2266: f64, t1580: f64, t3635: f64, t1073: f64, t1557: f64, t1559: f64, t8654: f64) -> (f64, f64, f64, f64) {
    let t12174 = 4.0_f64 / 9.0_f64 * t8675 * t3637;
    let t12175 = t3653 * t358;
    let t12176 = t12175 * t363;
    let t12177 = t2266 * t12176;
    let t12181 = t2266 * t3635 * t1580;
    let t12184 = t1073 * t1557;
    let t12186 = t8654 * t12184 * t1559;
    (t12174, t12177, t12181, t12186)
}
