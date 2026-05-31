//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2198/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2198<F: Float>(t101435: F, t2035: F, t28196: F, t28197: F, t75365: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F) -> (F, F, F, F, F, F) {
    let t101436 = t101435 * t2035;
    let t101439 = F::cast_from(4.0_f64) * t28196 * t28197 * t75365;
    let t101448 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t94976;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    let t101454 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t101453;
    let t101455 = t25823 * t4287;
    (t101436, t101439, t101448, t101451, t101454, t101455)
}
