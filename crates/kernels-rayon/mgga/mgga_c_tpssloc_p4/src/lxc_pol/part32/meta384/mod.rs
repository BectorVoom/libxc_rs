//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1454;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1455;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1456;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta384(t16684: f64, t16686: f64, t16698: f64, t16720: f64, t225: f64, t1504: f64, t68: f64, t1891: f64, t5527: f64, t776: f64, t4119: f64, t4226: f64, t5544: f64, t845: f64, t16662: f64, t824: f64, t1506: f64, t228: f64, t230: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t5601: f64, t5605: f64, t5608: f64, t822: f64, t825: f64, t232: f64, t860: f64, t2732: f64, t5612: f64, t1509: f64, t1519: f64, t829: f64, t4234: f64, t4282: f64, t5550: f64, t9573: f64, t213: f64, t221: f64, t4128: f64, t12986: f64, t13002: f64, t13005: f64, t13010: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9547: f64, t9572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16723, t16729, t16737, t16740) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1454(t16684, t16686, t16698, t16720, t225, t1504, t68, t1891, t5527, t776, t4119, t4226);
        let t16752 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1455(t5544, t845, t776, t16662, t824, t1504, t1506, t16723, t16729, t16737, t16740, t228, t230, t4219, t4225, t4227, t4230, t5601, t5605, t5608, t822, t825);
        let (t16753, t16754, t16756, t16758, t16759, t16762, t16769) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1456(t16752, t232, t860, t2732, t5612, t1509, t1519, t829, t4234, t4282, t5550, t9573);
        let t16781 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1457(t213, t5527, t221, t776, t4119, t4128, t12986, t13002, t13005, t13010, t16769, t4127, t9526, t9540, t9542, t9547, t9572);
    (t16752, t16753, t16754, t16756, t16758, t16759, t16762, t16781)
}
