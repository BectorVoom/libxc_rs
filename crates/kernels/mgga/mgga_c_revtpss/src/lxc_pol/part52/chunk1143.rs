//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1143/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1143<F: Float>(t198: F, t206: F, t8656: F, t11064: F, t1032: F, t7398: F, t867: F, t7060: F, t7063: F, t28425: F, t8479: F, t25386: F) -> (F, F, F, F, F, F, F) {
    let t121751 = t198 * t206 * t8656;
    let t121793 = t8656 * t11064;
    let t121803 = t7398 * t1032;
    let t121804 = t121803 * t867;
    let t121806 = t7063 * t121804 * t7060;
    let t121808 = t8479 * t28425;
    let t121809 = t25386 * t121808;
    (t121751, t121793, t121803, t121804, t121806, t121808, t121809)
}
