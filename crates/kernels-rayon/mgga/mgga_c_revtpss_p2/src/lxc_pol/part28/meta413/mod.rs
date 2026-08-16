//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1561;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1562;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1563;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1564;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1565;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1566;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta413(t15283: f64, t953: f64, t1622: f64, t2944: f64, t1634: f64, t2988: f64, t15127: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64, t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11339: f64, t11366: f64, t11368: f64, t11422: f64, t11423: f64, t15221: f64, t15230: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15125: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t954: f64, t4682: f64, t964: f64, t11404: f64, t11409: f64, t11507: f64, t11548: f64, t15263: f64, t15267: f64, t15274: f64, t15277: f64, t15280: f64, t2943: f64, t2968: f64, t3007: f64, t3012: f64, t4652: f64, t4674: f64, t4685: f64, t946: f64, t974: f64, t1626: f64, t3011: f64, t11574: f64, t11528: f64, t4595: f64, t11294: f64, t4636: f64, t4632: f64, t934: f64, t2874: f64, t1610: f64, t2918: f64, t2875: f64, t4635: f64, t11299: f64, t2926: f64, t4631: f64, t2924: f64, t11387: f64, t1609: f64, t11385: f64, t4644: f64, t945: f64, t11456: f64, t2982: f64, t3015: f64, t311: f64, t4708: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15284, t15287, t15290, t15301, t15315) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1560(t15283, t953, t1622, t2944, t1634, t2988, t15127, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
        let (t15322, t15324, t15337) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1561(t15191, t15197, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11422, t11423, t15221, t15230);
        let t15339 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1562(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15301, t15315, t15322, t15324, t15337);
        let t15348 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1563(t15339, t954, t4682, t964, t11404, t11409, t11507, t11548, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t2943, t2968, t3007, t3012, t4652, t4674, t4685, t946, t974);
        let (t15350, t15373) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1564(t1626, t3011, t15125, t15191, t11134, t11136, t11138, t11140, t11574, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15377, t15379, t15382, t15385, t15388) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1565(t11528, t4595, t11294, t4636, t4632, t934, t2874, t1610, t2918, t2875, t4635, t11299);
        let (t15392, t15395, t15399, t15400) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1566(t2926, t4631, t934, t2924, t2918, t4635, t11387, t1609, t2875, t11385, t4644, t945);
        let t15403 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1567(t11456, t15350, t15373, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15400, t1634, t2982, t3015, t311, t4708, t955);
    (t15348, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15403)
}
