//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 337/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk337(t2679: f64, t2685: f64, t2684: f64, t1589: f64, t948: f64, t1628: f64, t965: f64, t2586: f64, t531: f64, t2530: f64, t808: f64, t568: f64) -> (f64, f64, f64, f64, f64) {
    let t2686 = t2685 * t2679;
    let t2687 = t2684 * t2686;
    let t2689 = t1589 * t948;
    let t2692 = t1628 * t965;
    let t2699 = t531 * t2586;
    let t2704 = t808 * t2530;
    let t2705 = t568 * t2704;
    (t2687, t2689, t2692, t2699, t2705)
}
