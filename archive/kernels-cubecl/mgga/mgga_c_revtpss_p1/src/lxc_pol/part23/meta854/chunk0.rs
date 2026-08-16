//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2741/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741<F: Float>(t17290: F, t5362: F, t17435: F, t5327: F, t3655: F, t6595: F, t1256: F, t21313: F, t21316: F, t1261: F, t20272: F, t247: F, t3634: F) -> (F, F, F, F, F, F) {
    let t71740 = t17290 * t5362;
    let t71742 = t5327 * t17435;
    let t71744 = t6595 * t3655;
    let t71749 = t21313 * t1256;
    let t71751 = t21316 * t1256;
    let t71827 = t1261 * t247 * t3634 * t20272;
    (t71740, t71742, t71744, t71749, t71751, t71827)
}
