//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1217/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1217(t18750: f64, t219: f64, t5832: f64, t18000: f64, t1805: f64, t2407: f64, t768: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t18751 = param_beta * t18750;
    let t18753 = t5832 * t219;
    let t18767 = t18000 * t1805 * t2407;
    let t18770 = t768 * t1805;
    (t18751, t18753, t18767, t18770)
}
