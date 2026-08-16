//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta593(t5611: f64, t852: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64, t17095: f64, t17098: f64, t18287: f64, t1176: f64, t1714: f64, t19256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59331, t59466, t59498, t59503, t59519, t59537, t64595, t64825, t65203) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1981(t5611, t852, t17100, t225, t17087, t17060, t17095, t17098, t18287, t1176, t1714, t19256);
    (t59331, t59466, t59498, t59503, t59519, t59537, t64595, t64825, t65203)
}
