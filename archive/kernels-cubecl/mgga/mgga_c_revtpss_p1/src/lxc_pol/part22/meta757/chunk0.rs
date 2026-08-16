//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2835/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2835<F: Float>(t1063: F, t247: F, t42778: F, t906: F, t11986: F, t2858: F, t373: F, t675: F, t828: F) -> (F, F, F, F) {
    let t42781 = t1063 * t247 * t42778 * t906;
    let t42785 = t1063 * t247 * t11986 * t2858;
    let t42792 = t675 * t373;
    let t42793 = t828 * t42792;
    (t42781, t42785, t42792, t42793)
}
