//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2261/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261<F: Float>(t13517: F, t196: F, t197: F, t2035: F, t28196: F, t28197: F, t75365: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F) -> (F, F, F, F, F) {
    let t101435 = t13517 * t196 * t197;
    let t101436 = t101435 * t2035;
    let t101439 = F::new(4.0) * t28196 * t28197 * t75365;
    let t101448 = F::new(22.0) / F::new(9.0) * t94976;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    (t101436, t101439, t101448, t101451, t101453)
}
