//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1187/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1187<F: Float>(t5: F, t1923: F, t2123: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7583 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t6954 * t2123 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t6960 + t6963 * t2123 / F::cast_from(3.0_f64) - t1923 * t7576 / F::cast_from(6.0_f64) - t1923 * t7579 / F::cast_from(6.0_f64));
    let t7584 = t7583 * t117;
    (t7583, t7584)
}
