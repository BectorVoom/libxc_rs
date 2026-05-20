//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2738/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2738<F: Float>(t17789: F, t21017: F, t12916: F, t17747: F, t20962: F, t3717: F, t70994: F, t1261: F, t20867: F, t3172: F, t12956: F, t20783: F) -> (F, F, F, F, F) {
    let t71476 = t21017 * t17789;
    let t71490 = t17747 * t12916 * t20962;
    let t71513 = t3717 * t70994;
    let t71539 = t1261 * t3172 * t20867;
    let t71541 = t12956 * t20783;
    (t71476, t71490, t71513, t71539, t71541)
}
