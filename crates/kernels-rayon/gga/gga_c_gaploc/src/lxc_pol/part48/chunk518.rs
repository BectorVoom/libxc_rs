//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 518/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk518(t2685: f64, t9829: f64, t2684: f64, t2465: f64, t2581: f64, t2464: f64, t3311: f64, t7416: f64, t2013: f64, t3296: f64, t969: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t9830 = t2685 * t9829;
    let t9831 = t2684 * t9830;
    let t9832 = 0.38342925953920749676e0_f64 * t9831;
    let t9833 = t2465 * t2581;
    let t9834 = t2464 * t9833;
    let t9835 = t2684 * t9834;
    let t9836 = 0.85206502119823888169e-1_f64 * t9835;
    let t9837 = t7416 * t3311;
    let t9838 = 0.38342925953920749676e0_f64 * t9837;
    let t9845 = t2013 * t3296;
    let t9846 = 0.38342925953920749676e0_f64 * t9845;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    (t9832, t9836, t9838, t9846, t9848)
}
