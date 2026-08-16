//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk978;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta283(t20936: f64, t225: f64, t237: f64, t119: f64, t20756: f64, t210: f64, t1484: f64, t5544: f64, t2701: f64, t820: f64, t20870: f64, t819: f64, t13283: f64, t1512: f64, t1516: f64, t16872: f64, t16976: f64, t20904: f64, t20908: f64, t249: f64, t4172: f64, t5587: f64, t5624: f64, t5628: f64, t817: f64, t843: f64, t9559: f64, t9974: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20937, t20938, t20944, t20947, t20949, t20953) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk978(t20936, t225, t237, t119, t20756, t210, t1484, t5544, t2701, t820, t20870, t819);
        let t20958 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk979(t13283, t1512, t1516, t16872, t16976, t20904, t20908, t20938, t20944, t20949, t20953, t249, t4172, t5587, t5624, t5628, t817, t843, t9559, t9974);
    (t20937, t20938, t20944, t20947, t20949, t20953, t20958)
}
