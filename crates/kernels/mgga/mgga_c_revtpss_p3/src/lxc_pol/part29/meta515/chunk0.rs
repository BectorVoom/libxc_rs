//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1837/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837<F: Float>(t233: F, t41077: F, t1955: F, t92888: F, t7056: F, t9646: F, t1954: F, t39643: F, t2453: F, t25309: F, t25304: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t93118 = t41077 * t233;
    let t93126 = t1955 * t92888;
    let t93134 = t9646 * t7056;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93157 = t2453 * t25309;
    let t93160 = t25304 * t25309;
    let t93169 = t2453 * t251;
    (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169)
}
