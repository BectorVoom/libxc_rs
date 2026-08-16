//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2229;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2230;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2231;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2232;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta592(t1609: f64, t19330: f64, t2924: f64, t1622: f64, t6173: f64, t11452: f64, t23705: f64, t23451: f64, t3014: f64, t11574: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t11560: f64, t324: f64, t11409: f64, t11450: f64, t11466: f64, t15350: f64, t15406: f64, t1634: f64, t19156: f64, t23665: f64, t23723: f64, t23755: f64, t23758: f64, t23761: f64, t23764: f64, t23769: f64, t2943: f64, t2968: f64, t2987: f64, t3012: f64, t311: f64, t4685: f64, t6177: f64, t6206: f64, t6209: f64, t946: f64, t23720: f64, t300: f64, t23455: f64, t23459: f64, t23562: f64, t23564: f64, t23567: f64, t23570: f64, t23698: f64, t23652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23770, t23772, t23773, t23776, t23785, t23798) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2229(t1609, t19330, t2924, t1622, t6173, t11452, t23705, t23451, t3014, t11574, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let (t23811, t23812) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2230(t11560, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505, t324);
        let t23814 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2231(t11409, t11450, t11466, t15350, t15406, t1634, t19156, t23665, t23723, t23755, t23758, t23761, t23764, t23769, t23772, t23773, t23776, t23785, t23798, t23812, t2943, t2968, t2987, t3012, t311, t4685, t6177, t6206, t6209, t946);
        let (t23816, t23818, t23819) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2232(t23720, t23814, t300, t23812, t23455, t23459, t23562, t23564, t23567, t23570, t23665, t23698, t23769, t23772);
        let t23820 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2233(t23652, t23819);
    (t23770, t23772, t23773, t23776, t23785, t23798, t23811, t23816, t23818, t23820)
}
