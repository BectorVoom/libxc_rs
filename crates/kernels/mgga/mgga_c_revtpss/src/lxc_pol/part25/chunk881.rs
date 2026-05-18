//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 881/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk881<F: Float>(t10194: F, t10259: F, t10415: F, t10416: F, t1312: F, t2322: F, t2371: F, t5523: F, t670: F, t2389: F, t705: F, t707: F) -> (F, F) {
    let t10426 = F::new(2.0) * t10259 * t1312 + F::new(6.0) * t10416 * t670 + F::new(6.0) * t2322 * t2371 + F::new(6.0) * t2371 * t5523 + F::new(6.0) * t10194 + t10415;
    let t10428 = t705 * t2389;
    let t10430 = F::new(12.0) * t10428 * t707;
    (t10426, t10430)
}
