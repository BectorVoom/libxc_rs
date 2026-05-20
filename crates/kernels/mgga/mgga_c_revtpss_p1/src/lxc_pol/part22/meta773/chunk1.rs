//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2860/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2860<F: Float>(t12808: F, t17350: F, t12865: F, t12909: F, t13037: F, t472: F, t3603: F, t482: F, t675: F, t828: F) -> (F, F, F, F, F, F) {
    let t44517 = t12808 * t17350;
    let t44521 = t12909 * t12865;
    let t44531 = F::new(1.0) / t13037 / t472;
    let t44535 = t3603 * t3603;
    let t44545 = t675 * t482;
    let t44546 = t828 * t44545;
    (t44517, t44521, t44531, t44535, t44545, t44546)
}
