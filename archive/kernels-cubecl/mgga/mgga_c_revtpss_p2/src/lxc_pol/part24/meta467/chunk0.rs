//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1442/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442<F: Float>(t14472: F, t1580: F, t2439: F, t136: F, t2457: F, t41011: F, t6048: F, t10504: F, t6071: F, t18317: F, t2435: F, t10815: F, t6019: F) -> (F, F, F, F, F) {
    let t61400 = t2439 * t14472 * t1580;
    let t61407 = t41011 * t6048 * t136 * t2457;
    let t61411 = t10504 * t6071 * t136 * t2457;
    let t61448 = t2435 * t18317;
    let t61570 = t10815 * t6019;
    (t61400, t61407, t61411, t61448, t61570)
}
