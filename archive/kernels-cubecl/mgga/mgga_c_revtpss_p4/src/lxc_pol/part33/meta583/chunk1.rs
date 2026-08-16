//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1996/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1996<F: Float>(t233: F, t41077: F, t7056: F, t9646: F, t1949: F, t22: F, t25402: F, t1954: F, t39643: F, t2470: F, t25295: F, t7058: F) -> (F, F, F, F, F, F) {
    let t93118 = t41077 * t233;
    let t93134 = t9646 * t7056;
    let t93136 = t25402 * t1949 * t22;
    let t93138 = F::cast_from(0.43639970290213137151e-3_f64) * t93134 * t93136;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93142 = F::cast_from(0.51727911450665971904e-3_f64) * t93140 * t93136;
    let t93150 = t25295 * t2470;
    let t93151 = t7058 * t93150;
    (t93118, t93138, t93139, t93142, t93150, t93151)
}
