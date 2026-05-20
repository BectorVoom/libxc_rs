//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1462/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1462<F: Float>(t41606: F, t41621: F, t41637: F, t41652: F, t915: F, t935: F, t315: F, t41235: F, t11449: F, t941: F, t2941: F, t2966: F, t302: F) -> (F, F, F, F) {
    let t41657 = F::new(1.0) * t915 * (t41606 + t41621 + t41637 + t41652) * t935;
    let t41658 = t315 * t41235;
    let t41662 = t941 * t11449;
    let t41667 = t302 / t2966 / t2941;
    (t41657, t41658, t41662, t41667)
}
