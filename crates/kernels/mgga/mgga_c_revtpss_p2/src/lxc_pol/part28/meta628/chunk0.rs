//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2259/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259<F: Float>(t5: F, t101152: F, t101185: F, t101225: F, t101259: F, t101309: F, t101340: F, t101371: F, t101402: F, t117: F, t2014: F, t25177: F, t7934: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t101406 = piecewise3::<F>(t8, F::new(0.0), t101152 + t101185 + t101225 + t101259 + t101309 + t101340 + t101371 + t101402);
    let t101407 = t101406 * t117;
    let t101416 = F::new(2.0) * t2014 * t7934 * t25177;
    (t101407, t101416)
}
