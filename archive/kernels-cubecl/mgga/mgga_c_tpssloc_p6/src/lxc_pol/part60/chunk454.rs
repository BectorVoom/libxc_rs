//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 454/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk454<F: Float>(t252: F, t5558: F, t1492: F, t1519: F, t119: F, t5527: F, t210: F, t5544: F, t225: F) -> (F, F, F, F, F) {
    let t5559 = t5558 * t252;
    let t5561 = t1492 * t1519;
    let t5567 = t119 * t5527;
    let t5568 = t210 * t5567;
    let t5571 = t119 * t5544;
    let t5572 = t210 * t5571;
    let t5575 = t5558 * t225;
    (t5559, t5561, t5568, t5572, t5575)
}
