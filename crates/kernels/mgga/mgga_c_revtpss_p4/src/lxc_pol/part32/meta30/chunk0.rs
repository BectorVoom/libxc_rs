//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 202/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk202<F: Float>(t572: F, t573: F, t10: F, t2: F, t17: F, t16: F, t3: F) -> (F, F, F, F, F) {
    let t575 = t572 * t573 + F::cast_from(1.0_f64);
    let t576 = t10 * t2;
    let t578 = F::cast_from(2.0_f64) * t576 * t17;
    let t579 = t16 * t3;
    let t580 = F::cast_from(1.0_f64) / t579;
    (t575, t576, t578, t579, t580)
}
