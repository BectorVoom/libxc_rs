//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1970/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1970<F: Float>(t114: F, t1513: F, t25823: F, t665: F, t25826: F, t4287: F, t6998: F, t25822: F, t25824: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t28034 = t25823 * t1513;
    let t28036 = t1513 * t665;
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    let t28042 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t25822 + t25824 / F::cast_from(3.0_f64) + t28034 / F::cast_from(3.0_f64) + t28037 / F::cast_from(4.0_f64) - t28039 / F::cast_from(8.0_f64));
    (t28036, t28042)
}
