//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta702(t1227: f64, t13969: f64, t18593: f64, t15640: f64, t15737: f64, t15503: f64, t19025: f64, t3535: f64, t1202: f64, t19032: f64, t15498: f64, t4993: f64, t15486: f64, t5024: f64, t15590: f64, t5018: f64, t15507: f64, t15548: f64, t19057: f64, t3506: f64, t15438: f64, t15569: f64, t15608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66084, t66092, t66120, t66147, t66150, t66153) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288(t1227, t13969, t18593, t15640, t15737, t15503, t19025, t3535, t1202, t19032, t15498, t4993);
        let (t66155, t66159, t66165, t66241, t66255, t66268) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289(t15486, t5024, t15590, t5018, t15507, t15548, t13969, t19057, t3506, t15438, t15569, t15608);
    (t66084, t66092, t66120, t66147, t66150, t66153, t66155, t66159, t66165, t66241, t66255, t66268)
}
