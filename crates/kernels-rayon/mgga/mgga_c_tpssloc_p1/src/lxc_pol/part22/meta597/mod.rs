//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2118;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta597(t13797: f64, t1597: f64, t13783: f64, t340: f64, t4548: f64, t698: f64, t973: f64, t10224: f64, t4522: f64, t13895: f64, t2960: f64, t1599: f64, t2402: f64, t13908: f64, t2986: f64, t344: f64, t43052: f64, t4343: f64, t2978: f64, t4338: f64, t697: f64, t43053: f64, t4514: f64, t1592: f64, t42891: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48221, t48279, t48293, t48321, t48329, t48336) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2118(t13797, t1597, t13783, t340, t4548, t698, t973, t10224, t4522, t13895, t2960, t1599, t2402);
        let (t48339, t48374, t48379, t48382, t48397) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2119(t13908, t2960, t2986, t344, t43052, t4343, t2978, t4338, t697, t43053, t4514, t1592, t42891, t973);
    (t48221, t48279, t48293, t48321, t48329, t48336, t48339, t48374, t48379, t48382, t48397)
}
