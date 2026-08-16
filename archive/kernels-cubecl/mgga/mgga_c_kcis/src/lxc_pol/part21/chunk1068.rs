//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1068/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1068<F: Float>(t233: F, t26664: F, t7673: F, t7676: F, t7679: F, t380: F, t982: F) -> (F, F, F, F) {
    let t26665 = t233 * t26664;
    let t26666 = t26665 / F::cast_from(8.0_f64);
    let t26667 = t7673 * t7676;
    let t26668 = t26667 / F::cast_from(8.0_f64);
    let t26669 = t7673 * t7679;
    let t26670 = t26669 / F::cast_from(8.0_f64);
    let t26671 = t380 * t982;
    (t26666, t26668, t26670, t26671)
}
