//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 564/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk564(t10556: f64, t544: f64, t2392: f64, t10506: f64, t10508: f64, t10510: f64, t10512: f64, t10516: f64, t10519: f64, t10522: f64, t10529: f64, t10536: f64, t10539: f64, t10542: f64, t10545: f64, t10549: f64, t10551: f64, t10554: f64) -> (f64, f64) {
    let t10557 = t544 * t10556;
    let t10559 = 0.42900587942220512003e1_f64 * t10557 * t2392;
    let t10560 = t10506 + t10508 + t10510 + t10512 - t10516 + t10519 - t10522 - t10529 + t10536 + t10539 + t10542 + t10545 - t10549 - t10551 + t10554 + t10559;
    (t10557, t10560)
}
