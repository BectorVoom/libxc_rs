//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1374/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1374<F: Float>(t5: F, t116759: F, t116798: F, t116821: F, t116844: F, t117: F, t111696: F, t114372: F, t114375: F, t114377: F, t114380: F, t114382: F, t114384: F, t114387: F, t114389: F, t114391: F, t114403: F, t116732: F, t1518: F, t22633: F, t29427: F, t34446: F, t5920: F, t7586: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t116847 = piecewise3::<f64>(t8, F::new(0.0), t116759 + t116798 + t116821 + t116844);
    let t116848 = t116847 * t117;
    let t116861 = F::new(6.0) * t111696 * t1518 + F::new(2.0) * t22633 * t7586 + F::new(6.0) * t29427 * t5920 + F::new(6.0) * t34446 * t5920 + t114372 + t114375 + t114377 + t114380 + t114382 + t114384 + t114387 + t114389 + t114391 + t114403 + F::new(6.0) * t116732 + t116848;
    (t116848, t116861)
}
