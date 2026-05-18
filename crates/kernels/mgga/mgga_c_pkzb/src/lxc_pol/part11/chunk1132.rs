//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1132/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1132<F: Float>(t2026: F, t3640: F, t5939: F, t154: F, t18086: F, t276: F, t3542: F, t735: F, t9546: F, t9583: F, t3515: F, t5688: F) -> (F, F, F, F, F) {
    let t25189 = t2026 * t5939 * t3640;
    let t25198 = t276 * t154 * t18086 * t3542;
    let t25207 = t735 * t9546;
    let t25212 = t735 * t9583;
    let t25218 = t276 * t154 * t5688 * t3515;
    (t25189, t25198, t25207, t25212, t25218)
}
