//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 688/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk688<F: Float>(t5: F, t1923: F, t2048: F, t6954: F, t6960: F, t6963: F, t7343: F, t7351: F, t7352: F, t117: F, t116: F, t2051: F) -> (F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t7356 = piecewise3::<f64>(t8, F::new(0.0), t6954 * t2048 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t7343 * t6960 - F::new(2.0) / F::new(3.0) * t6963 * t2048 - t7351 + t1923 * t7352 / F::new(3.0));
    let t7357 = t7356 * t117;
    let t7359 = t2051 * t116;
    (t7356, t7357, t7359)
}
