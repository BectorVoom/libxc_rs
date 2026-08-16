//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2065;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2066;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta485<F: Float>(t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11422: F, t11423: F, t15221: F, t15230: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t15301: F, t15315: F, t954: F, t4682: F, t964: F, t11404: F, t11409: F, t11507: F, t11548: F, t15263: F, t15267: F, t15274: F, t15277: F, t15280: F, t15284: F, t15287: F, t15290: F, t2943: F, t2968: F, t3007: F, t3012: F, t4652: F, t4674: F, t4685: F, t946: F, t974: F, t1626: F, t3011: F, t11574: F, t15127: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F) -> (F, F, F, F, F, F) {
        let (t15322, t15324, t15337) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2064::<F>(t15191, t15197, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11422, t11423, t15221, t15230);
        let t15339 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2065::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15301, t15315, t15322, t15324, t15337);
        let (t15340, t15343, t15348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2066::<F>(t15339, t954, t4682, t964, t11404, t11409, t11507, t11548, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t2943, t2968, t3007, t3012, t4652, t4674, t4685, t946, t974);
        let (t15350, t15373) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2067::<F>(t1626, t3011, t15125, t15191, t11134, t11136, t11138, t11140, t11574, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15339, t15340, t15343, t15348, t15350, t15373)
}
