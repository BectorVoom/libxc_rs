//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 893/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk893<F: Float>(t10563: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t9333: F, t9394: F) -> (F, F, F, F, F) {
    let t10564 = F::new(3.0) * t10563;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    let t10568 = F::cast_from(0.56968947174242584612e-3_f64) * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10570 = F::cast_from(0.73245789224026180216e-3_f64) * t10569;
    let t10571 = t9333 - t10552 + t10554 + t10557 + t9394 + t10560 + t10562 + t10564 + t10566 - t10568 + t10570;
    (t10564, t10566, t10568, t10570, t10571)
}
