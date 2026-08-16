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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1561;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1562;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1563;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1564;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1565;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1566;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta413<F: Float>(t15283: F, t953: F, t1622: F, t2944: F, t1634: F, t2988: F, t15127: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F, t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11422: F, t11423: F, t15221: F, t15230: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t954: F, t4682: F, t964: F, t11404: F, t11409: F, t11507: F, t11548: F, t15263: F, t15267: F, t15274: F, t15277: F, t15280: F, t2943: F, t2968: F, t3007: F, t3012: F, t4652: F, t4674: F, t4685: F, t946: F, t974: F, t1626: F, t3011: F, t11574: F, t11528: F, t4595: F, t11294: F, t4636: F, t4632: F, t934: F, t2874: F, t1610: F, t2918: F, t2875: F, t4635: F, t11299: F, t2926: F, t4631: F, t2924: F, t11387: F, t1609: F, t11385: F, t4644: F, t945: F, t11456: F, t2982: F, t3015: F, t311: F, t4708: F, t955: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15284, t15287, t15290, t15301, t15315) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1560::<F>(t15283, t953, t1622, t2944, t1634, t2988, t15127, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
        let (t15322, t15324, t15337) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1561::<F>(t15191, t15197, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11422, t11423, t15221, t15230);
        let t15339 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1562::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15301, t15315, t15322, t15324, t15337);
        let t15348 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1563::<F>(t15339, t954, t4682, t964, t11404, t11409, t11507, t11548, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t2943, t2968, t3007, t3012, t4652, t4674, t4685, t946, t974);
        let (t15350, t15373) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1564::<F>(t1626, t3011, t15125, t15191, t11134, t11136, t11138, t11140, t11574, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15377, t15379, t15382, t15385, t15388) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1565::<F>(t11528, t4595, t11294, t4636, t4632, t934, t2874, t1610, t2918, t2875, t4635, t11299);
        let (t15392, t15395, t15399, t15400) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1566::<F>(t2926, t4631, t934, t2924, t2918, t4635, t11387, t1609, t2875, t11385, t4644, t945);
        let t15403 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1567::<F>(t11456, t15350, t15373, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15400, t1634, t2982, t3015, t311, t4708, t955);
    (t15348, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15403)
}
