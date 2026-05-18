//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 761/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk761<F: Float>(t5: F, t572: F, t7953: F, t2047: F, t7719: F, t1923: F, t2048: F, t7343: F, t7351: F, t7702: F, t7706: F, t7709: F, t117: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t7955 = F::new(3.0) * t572 * t7953;
    let t7964 = t2047 * t7719;
    let t7968 = piecewise3::<f64>(t8, F::new(0.0), t7702 * t2048 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t7343 * t7706 - F::new(2.0) / F::new(3.0) * t7709 * t2048 - t7351 + t1923 * t7964 / F::new(3.0));
    let t7969 = t7968 * t117;
    (t7955, t7964, t7968, t7969)
}
