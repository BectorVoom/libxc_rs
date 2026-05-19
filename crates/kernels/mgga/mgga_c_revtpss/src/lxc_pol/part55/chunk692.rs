//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 692/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk692<F: Float>(t5: F, t1923: F, t7349: F, t2047: F, t6977: F, t2048: F, t6954: F, t6960: F, t6963: F, t7343: F, t117: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7351 = F::new(8.0) / F::new(9.0) * t1923 * t7349;
    let t7352 = t2047 * t6977;
    let t7356 = piecewise3::<F>(t8, F::new(0.0), t6954 * t2048 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t7343 * t6960 - F::new(2.0) / F::new(3.0) * t6963 * t2048 - t7351 + t1923 * t7352 / F::new(3.0));
    let t7357 = t7356 * t117;
    (t7351, t7352, t7356, t7357)
}
