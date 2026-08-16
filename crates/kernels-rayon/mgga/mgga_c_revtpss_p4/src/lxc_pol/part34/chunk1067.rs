//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1067/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1067(t1196: f64, t24765: f64, t24255: f64, t24257: f64, t24259: f64, t24261: f64, t24482: f64, t24484: f64, t24490: f64, t24496: f64, t24500: f64, t24763: f64) -> (f64, f64) {
    let t24767 = 0.10254018858216406658e4_f64 * t1196 * t24765;
    let t24768 = t24490 + t24496 - t24500 + t24763 - t24767 - t24482 + t24255 - t24484 + t24257 + t24259 + t24261;
    (t24767, t24768)
}
