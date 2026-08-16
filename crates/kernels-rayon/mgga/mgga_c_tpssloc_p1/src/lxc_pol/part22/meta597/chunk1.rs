//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2119/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2119(t13908: f64, t2960: f64, t2986: f64, t344: f64, t43052: f64, t4343: f64, t2978: f64, t4338: f64, t697: f64, t43053: f64, t4514: f64, t1592: f64, t42891: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t48338 = t2960 * t13908;
    let t48339 = 0.14814814814814814814e-2_f64 * t48338;
    let t48373 = t2986 * t43052 * t344 * t4343;
    let t48374 = 0.37037037037037037036e-3_f64 * t48373;
    let t48378 = t2986 * t697 * t2978 * t344 * t4338;
    let t48379 = 0.24691358024691358024e-3_f64 * t48378;
    let t48381 = t2986 * t43053 * t4514;
    let t48382 = 0.18518518518518518518e-3_f64 * t48381;
    let t48397 = t973 * t42891 * t1592;
    (t48339, t48374, t48379, t48382, t48397)
}
