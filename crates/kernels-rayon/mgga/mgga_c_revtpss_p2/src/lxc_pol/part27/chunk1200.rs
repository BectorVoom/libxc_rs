//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1200/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1200(t2439: f64, t25334: f64, t887: f64, t2722: f64, t7048: f64, t10799: f64, t27261: f64, t10773: f64, t25270: f64, t10766: f64, t10794: f64, t7036: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92935 = t2439 * t25334 * t887;
    let t92937 = t7048 * t2722;
    let t92942 = t27261 * t10799;
    let t92944 = t25270 * t10773;
    let t92946 = t25270 * t10766;
    let t92948 = t25270 * t10794;
    let t92951 = t820 * t7036 * t844;
    (t92935, t92937, t92942, t92944, t92946, t92948, t92951)
}
