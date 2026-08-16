//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1694;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1695;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta401(t248: f64, t3521: f64, t5975: f64, t1227: f64, t1409: f64, t15701: f64, t15700: f64, t3578: f64, t1735: f64, t4729: f64, t18232: f64, t4900: f64, t3450: f64, t5398: f64, t3449: f64, t18237: f64, t4908: f64, t3448: f64, t6138: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18392, t18393, t18395, t18396, t18397, t18400, t18401, t18404) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1694(t248, t3521, t5975, t1227, t1409, t15701, t15700, t3578, t1735, t4729, t18232, t4900);
        let t18409 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1695(t3450, t5398);
        let (t18410, t18413, t18416) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1696(t18409, t3449, t18237, t4908, t3448, t6138);
    (t18392, t18393, t18395, t18396, t18397, t18400, t18401, t18404, t18409, t18410, t18413, t18416)
}
