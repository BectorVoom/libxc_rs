//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1149/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1149<F: Float>(t127: F, t371: F, t6337: F, t3205: F, t6276: F, t1025: F, t4845: F, t4858: F, t3172: F, t6307: F, t3150: F, t4820: F, t4879: F, t11947: F, t15745: F, t16134: F, t16160: F, t16190: F, t1665: F, t1671: F, t3188: F, t6327: F, t6339: F) -> (F,) {
    let t20016 = t371 * t127 * t6337;
    let t20017 = t3205 * t20016;
    let t20020 = t371 * t127 * t6276;
    let t20021 = t1025 * t20020;
    let t20025 = t4858 * t4845;
    let t20029 = t3172 * t6307;
    let t20030 = t3150 * t20029;
    let t20034 = t4879 * t4820;
    let t20036 = -0.22866142996303859718e-2 * t11947 * t6339 + 0.28582678745379824648e-3 * t20017 - 0.14291339372689912324e-3 * t20021 + 0.22866142996303859718e-2 * t15745 * t1665 - 0.28582678745379824648e-3 * t20025 + t16134 + 0.23818898954483187207e-3 * t3188 * t6327 + 0.28582678745379824648e-3 * t20030 - 0.22866142996303859718e-2 * t16190 * t1671 + 0.28582678745379824648e-3 * t20034 + t16160;
    (t20036,)
}
