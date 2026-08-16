//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1893;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta632(t1339: f64, t19732: f64, t6936: f64, t22779: f64, t28057: f64, t6371: f64, t80827: f64, t28073: f64, t80888: f64, t26301: f64, t7708: f64, t91208: f64, t26322: f64, t91202: f64, t20004: f64, t26309: f64, t19945: f64, t19981: f64, t22833: f64, t19994: f64, t221: f64, t26284: f64, t19631: f64, t1998: f64, t236: f64, t6926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97398, t97400, t97402, t97404, t97407) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1893(t1339, t19732, t6936, t22779, t28057, t6371, t80827, t28073, t80888, t26301, t7708, t91208);
        let (t97410, t97412, t97414, t97416, t97419, t97423) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1894(t26322, t7708, t91202, t20004, t26309, t19945, t19981, t22833, t19994, t221, t26284, t19631, t1998, t236, t6926);
    (t97398, t97400, t97402, t97404, t97407, t97410, t97412, t97414, t97416, t97419, t97423)
}
