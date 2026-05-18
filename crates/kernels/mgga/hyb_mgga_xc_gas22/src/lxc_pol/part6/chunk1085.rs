//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1085/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1085<F: Float>(t10571: F, t791: F, t10534: F, t10549: F, t6530: F, t6592: F, t8676: F, t8681: F, t789: F, t4117: F, t6601: F, t3329: F, t3335: F) -> (F, F, F, F, F, F) {
    let t10572 = t10571 * t791;
    let t10577 = -t6592 + F::new(4.0) / F::new(9.0) * t6530 + F::new(8.0) / F::new(9.0) * t8676 - t8681 - t10534 / F::new(3.0) + t10549;
    let t10578 = t789 * t10577;
    let t10584 = t6601 * t4117;
    let t10585 = t10584 * t791;
    let t10587 = t3335 * t3329;
    (t10572, t10577, t10578, t10584, t10585, t10587)
}
