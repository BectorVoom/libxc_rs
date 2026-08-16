//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2028/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2028(t11647: f64, t2141: f64, t24684: f64, t27634: f64, t461: f64, t607: f64, t1009: f64, t7324: f64, t24658: f64, t27635: f64, t3540: f64, t7334: f64) -> (f64, f64, f64, f64, f64) {
    let t86191 = t2141 * t11647 / 5184.0_f64;
    let t86234 = t27634 * t24684;
    let t86259 = t607 * t461;
    let t86261 = t7324 * t86259 * t1009;
    let t86264 = t24658 * t27635;
    let t86275 = t7334 * t3540;
    (t86191, t86234, t86261, t86264, t86275)
}
