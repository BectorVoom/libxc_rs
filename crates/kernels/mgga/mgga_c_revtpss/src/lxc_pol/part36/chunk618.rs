//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 618/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk618<F: Float>(t3014: F, t6226: F, t981: F, t3037: F, t4571: F, t6094: F, t6098: F, t6102: F, t341: F) -> (F, F, F, F) {
    let t6227 = t6226 * t3014;
    let t6229 = F::new(0.17315859105681463759e2) * t981 * t6227;
    let t6234 = t3037 + F::new(0.55555555555555555556e-2) * t4571 - F::new(0.55555555555555555555e-2) * t6094 + F::new(0.16666666666666666667e-1) * t6098 - F::new(0.83333333333333333333e-2) * t6102;
    let t6235 = t6234 * t341;
    (t6227, t6229, t6234, t6235)
}
