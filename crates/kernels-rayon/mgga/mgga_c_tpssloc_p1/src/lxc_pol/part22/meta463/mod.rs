//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1843;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta463(t15909: f64, t12044: f64, t12046: f64, t12048: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t12087: f64, t20372: f64, t20398: f64, t9780: f64, t9789: f64, t19682: f64, t15972: f64, t12094: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64) {
        let (t20520, t20521) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1843(t15909, t12044, t12046, t12048, t12053, t12055, t12057, t12059, t12087, t20372, t20398, t9780, t9789);
        let (t20523, t20524, t20525) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1844(t19682, t15972, t12094, t12103, t12105, t12109, t12114, t12116, t9793, t9797, t9820, t9824);
    (t20520, t20521, t20523, t20524, t20525)
}
