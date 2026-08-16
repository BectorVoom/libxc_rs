//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1070/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1070(t21040: f64, t6638: f64, t3626: f64, t471: f64, t5351: f64, t6429: f64, t6425: f64, t6421: f64, t12787: f64, t23842: f64, t5268: f64, t1042: f64) -> (f64, f64, f64, f64, f64) {
    let t24786 = t21040 * t6638;
    let t24787 = t3626 * t24786;
    let t24792 = t5351 * t471;
    let t24793 = t6429 * t24792;
    let t24794 = t3626 * t24793;
    let t24797 = t6425 * t24792;
    let t24798 = t3626 * t24797;
    let t24803 = t6421 * t24792;
    let t24804 = t12787 * t24803;
    let t24807 = t5268 * t23842;
    let t24808 = t1042 * t24807;
    (t24787, t24794, t24798, t24804, t24808)
}
