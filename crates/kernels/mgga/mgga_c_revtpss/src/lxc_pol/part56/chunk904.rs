//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 904/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk904<F: Float>(t5778: F, t9593: F, t243: F, t9794: F, t2246: F, t4171: F, t10308: F, t1466: F, t7063: F, t860: F, t1444: F, t543: F, t1419: F, t11239: F, t1269: F, t42859: F, t487: F) -> (F, F, F, F, F, F, F, F, F) {
    let t49575 = t5778 * t9593;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t93341 = t7063 * t860;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    let t96881 = t1269 * t11239;
    let t96886 = t487 * t42859;
    (t49575, t51076, t60221, t60224, t93341, t94396, t94801, t96881, t96886)
}
