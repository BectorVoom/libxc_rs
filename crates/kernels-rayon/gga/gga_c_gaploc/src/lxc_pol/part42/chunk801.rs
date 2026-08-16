//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 801/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk801(t2365: f64, t32357: f64, t6111: f64, t32436: f64, t24501: f64, t825: f64, t9438: f64, t33360: f64, t787: f64, t9824: f64, t33348: f64, t13141: f64, t2464: f64, t2684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43467 = t6111 * t2365 * t32357;
    let t43470 = t6111 * t2365 * t32436;
    let t43476 = t825 * t9438 * t24501;
    let t43522 = t787 * t33360 * t9824;
    let t43526 = t787 * t33348 * t9824;
    let t43581 = t2684 * t2464 * t13141;
    (t43467, t43470, t43476, t43522, t43526, t43581)
}
