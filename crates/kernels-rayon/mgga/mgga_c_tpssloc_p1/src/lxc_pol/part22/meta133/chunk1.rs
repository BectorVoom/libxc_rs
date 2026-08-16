//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 877/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk877(t182: f64, t4095: f64, t145: f64, t4094: f64, t185: f64, t1472: f64, t751: f64, t1409: f64) -> (f64, f64, f64, f64, f64) {
    let t4097 = 0.19751673498613801407e-1_f64 * t4095 * t182;
    let t4098 = t145 * t4094;
    let t4099 = t4098 * t185;
    let t4100 = t1472 * t751;
    let t4101 = t751 * t1409;
    (t4097, t4098, t4099, t4100, t4101)
}
