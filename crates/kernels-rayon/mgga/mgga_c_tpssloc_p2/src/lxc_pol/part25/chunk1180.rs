//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1180/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1180(t2319: f64, t7039: f64, t22550: f64, t7031: f64, t22549: f64, t2031: f64, t83728: f64, t83737: f64, t607: f64, t63: f64, t39054: f64, t7025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84149 = t7039 * t2319;
    let t84173 = t7031 * t22550;
    let t84174 = t22549 * t84173;
    let t84180 = t2031 * t83728;
    let t84183 = t2031 * t83737;
    let t84186 = t607 * t63;
    let t84190 = t39054 * t7025;
    (t84149, t84174, t84180, t84183, t84186, t84190)
}
