//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1000/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1000<F: Float>(t10732: F, t25227: F, t2661: F, t10700: F, t7045: F, t10705: F, t25234: F, t231: F, t92883: F, t233: F, t41077: F, t1955: F, t92888: F, t2828: F, t836: F, t7056: F, t9646: F) -> (F, F, F, F, F, F, F, F) {
    let t93091 = t2661 * t25227 * t10732;
    let t93093 = t7045 * t10700;
    let t93095 = t25234 * t10705;
    let t93104 = t92883 * t231;
    let t93118 = t41077 * t233;
    let t93126 = t1955 * t92888;
    let t93130 = t2828 * t836 * t231;
    let t93134 = t9646 * t7056;
    (t93091, t93093, t93095, t93104, t93118, t93126, t93130, t93134)
}
