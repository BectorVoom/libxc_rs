//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 709/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk709<F: Float>(t5: F, t1923: F, t1928: F, t6954: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F, t117: F, t116: F, t1931: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t6982 = piecewise3::<F>(t8, F::new(0.0), -t6954 * t1928 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6958 * t6960 + t6963 * t1928 / F::new(3.0) - t1923 * t6974 / F::new(6.0) - t1923 * t6978 / F::new(6.0));
    let t6983 = t6982 * t117;
    let t6985 = t1931 * t116;
    (t6982, t6983, t6985)
}
