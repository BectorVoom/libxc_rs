//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 996/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk996<F: Float>(t243: F, t247: F, t9949: F, t237: F, t236: F, t9646: F, t9721: F, t268: F, t207: F, t242: F, t240: F, t72: F) -> (F, F, F) {
    let t10685 = t9949 * t243 * t247;
    let t10687 = F::cast_from(0.37792653007779990369e-1_f64) * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = F::cast_from(0.20082057720118594944e-6_f64) * t10688 * t10690;
    let t10696 = F::cast_from(1.0_f64) / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    (t10687, t10692, t10698)
}
