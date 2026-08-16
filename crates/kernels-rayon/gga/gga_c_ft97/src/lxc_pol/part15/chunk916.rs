//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 916/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk916(t17837: f64, t4952: f64, t13519: f64, t5019: f64, t17831: f64, t3771: f64, t9523: f64, t1611: f64, t236: f64, t806: f64, t5045: f64, t626: f64, t701: f64) -> (f64, f64, f64, f64, f64) {
    let t65695 = t17837 * t4952;
    let t65702 = t13519 * t5019;
    let t65735 = t3771 * t17831 * t9523;
    let t65743 = t3771 * t236 * t1611 * t806;
    let t65850 = t701 * t626 * t5045;
    (t65695, t65702, t65735, t65743, t65850)
}
