//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3091/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091<F: Float>(t18502: F, t699: F, t18499: F, t136: F, t3297: F, t63394: F, t63386: F, t63390: F, t18509: F, t18507: F, t1113: F, t63410: F) -> (F, F, F, F, F, F, F, F) {
    let t64074 = t699 * t18502;
    let t64076 = t699 * t18499;
    let t64079 = t136 * t3297 * t63394;
    let t64082 = t136 * t3297 * t63386;
    let t64085 = t136 * t3297 * t63390;
    let t64087 = t699 * t18509;
    let t64089 = t699 * t18507;
    let t64092 = t136 * t1113 * t63410;
    (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092)
}
