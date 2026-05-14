//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 868/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk868<F: Float>(t536: F, t7768: F, t508: F, t509: F, t523: F, t521: F, t524: F, t7692: F, t1828: F, t2823: F) -> (F, F, F, F, F, F) {
    let t7769 = t536 * t7768;
    let t7773 = t509 * t508;
    let t7774 = t523 * t7773;
    let t7775 = t7774 * t521;
    let t7780 = t524 * t7692;
    let t7785 = t2823 * t1828;
    (t7769, t7773, t7774, t7775, t7780, t7785)
}
