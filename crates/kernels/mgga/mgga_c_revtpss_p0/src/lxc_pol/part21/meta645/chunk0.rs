//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2430/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2430<F: Float>(t2979: F, t3011: F, t11506: F, t960: F, t315: F, t41224: F, t2935: F, t2942: F, t11408: F, t941: F, t2986: F, t11465: F) -> (F, F, F, F, F, F, F) {
    let t41751 = t2979 * t3011;
    let t41756 = t960 * t11506;
    let t41759 = t315 * t41224;
    let t41775 = t2935 * t2942;
    let t41779 = t941 * t11408;
    let t41785 = t2979 * t2986;
    let t41788 = t960 * t11465;
    (t41751, t41756, t41759, t41775, t41779, t41785, t41788)
}
