//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 611/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk611(t2723: f64, t345: f64, t947: f64, t242: f64, t348: f64, t353: f64, t72: f64, t943: f64, t983: f64) -> (f64, f64, f64, f64, f64) {
    let t2732 = t2723 * t345;
    let t2733 = t947 * t2732;
    let t2734 = t242 * t2733;
    let t2737 = t348 * t353;
    let t2738 = t943 * t72;
    let t2740 = t983 * t2737 * t2738;
    (t2732, t2734, t2737, t2738, t2740)
}
