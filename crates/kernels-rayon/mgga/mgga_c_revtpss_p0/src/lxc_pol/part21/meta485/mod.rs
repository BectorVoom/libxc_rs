//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2065;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2066;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta485(t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11339: f64, t11366: f64, t11368: f64, t11422: f64, t11423: f64, t15221: f64, t15230: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15125: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t15301: f64, t15315: f64, t954: f64, t4682: f64, t964: f64, t11404: f64, t11409: f64, t11507: f64, t11548: f64, t15263: f64, t15267: f64, t15274: f64, t15277: f64, t15280: f64, t15284: f64, t15287: f64, t15290: f64, t2943: f64, t2968: f64, t3007: f64, t3012: f64, t4652: f64, t4674: f64, t4685: f64, t946: f64, t974: f64, t1626: f64, t3011: f64, t11574: f64, t15127: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15322, t15324, t15337) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2064(t15191, t15197, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11422, t11423, t15221, t15230);
        let t15339 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2065(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15301, t15315, t15322, t15324, t15337);
        let (t15340, t15343, t15348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2066(t15339, t954, t4682, t964, t11404, t11409, t11507, t11548, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t2943, t2968, t3007, t3012, t4652, t4674, t4685, t946, t974);
        let (t15350, t15373) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2067(t1626, t3011, t15125, t15191, t11134, t11136, t11138, t11140, t11574, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15339, t15340, t15343, t15348, t15350, t15373)
}
