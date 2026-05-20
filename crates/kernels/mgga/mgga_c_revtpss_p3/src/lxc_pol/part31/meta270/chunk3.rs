//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1211/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1211<F: Float>(t5: F, t1923: F, t1928: F, t6958: F, t7702: F, t7706: F, t7709: F, t7716: F, t7720: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7724 = piecewise3::<F>(t8, F::new(0.0), -t7702 * t1928 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6958 * t7706 + t7709 * t1928 / F::new(3.0) - t1923 * t7716 / F::new(6.0) - t1923 * t7720 / F::new(6.0));
    let t7725 = t7724 * t117;
    (t7724, t7725)
}
