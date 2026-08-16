//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 599/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk599(t2675: f64, t949: f64, t242: f64, t946: f64, t943: f64, t956: f64, t938: f64, t941: f64) -> (f64, f64, f64) {
    let t2676 = t2675 * t949;
    let t2677 = t242 * t2676;
    let t2678 = t946 * t2677;
    let t2680 = t956 * t943;
    let t2682 = t938 * t941 * t2680;
    (t2677, t2678, t2682)
}
