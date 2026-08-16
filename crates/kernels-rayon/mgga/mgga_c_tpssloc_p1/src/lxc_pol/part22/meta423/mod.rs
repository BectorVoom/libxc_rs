//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1738;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1739;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1740;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1741;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta423(t11665: f64, t11678: f64, t1174: f64, t11834: f64, t1218: f64, t15569: f64, t15717: f64, t15719: f64, t15722: f64, t15740: f64, t18997: f64, t19002: f64, t19005: f64, t19010: f64, t19016: f64, t19019: f64, t19026: f64, t3577: f64, t4889: f64, t4950: f64, t4954: f64, t4969: f64, t5046: f64, t6192: f64, t372: f64, t6163: f64, t479: f64, t471: f64, t248: f64, t3521: f64, t5979: f64, t1227: f64, t1009: f64, t6150: f64, t1011: f64, t1212: f64, t1226: f64, t6169: f64, t486: f64, t6218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t19029 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1738(t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t19016, t19019, t19026, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
        let (t19032, t19033) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1739(t372, t6163, t479, t471);
        let (t19040, t19041, t19045, t19046, t19047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1740(t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
        let t19051 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1741(t1226, t6169);
        let t19056 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1742(t486, t6218);
    (t19029, t19032, t19033, t19040, t19041, t19045, t19046, t19047, t19051, t19056)
}
