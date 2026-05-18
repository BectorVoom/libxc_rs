//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 196/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk196<F: Float>(t161: F, t815: F, t774: F, t755: F, t159: F, t8: F) -> (F, F, F, F, F) {
    let t816 = t815 * t161;
    let t818 = t161 * t774;
    let t819 = t755 * t818;
    let t821 = t159 * t8;
    let t822 = F::new(1.0) / t821;
    (t816, t818, t819, t821, t822)
}
