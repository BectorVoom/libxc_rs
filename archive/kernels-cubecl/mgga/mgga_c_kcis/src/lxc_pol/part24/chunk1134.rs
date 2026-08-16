//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1134/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1134<F: Float>(t2150: F, t9175: F, t2526: F, t755: F, t7627: F, t26553: F, t774: F, t8537: F, t8538: F, t153: F, t822: F, t2484: F, t26547: F) -> (F, F, F, F, F, F) {
    let t91841 = t9175 * t2150;
    let t91844 = t755 * t7627 * t2526;
    let t91847 = t755 * t26553 * t774;
    let t91850 = t8537 * t2150 * t8538;
    let t91852 = t153 * t822;
    let t91854 = t2484 * t26547;
    (t91841, t91844, t91847, t91850, t91852, t91854)
}
