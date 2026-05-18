//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 234/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk234<F: Float>(t863: F, t865: F, t315: F, t441: F, t323: F, t322: F, t463: F, t449: F) -> (F, F, F, F, F) {
    let t867 = F::new(0.13170898365871023197e1) * t863 * t865;
    let t868 = t315 * t441;
    let t869 = t868 * t323;
    let t871 = t322 * t463;
    let t872 = t449 * t871;
    (t867, t868, t869, t871, t872)
}
