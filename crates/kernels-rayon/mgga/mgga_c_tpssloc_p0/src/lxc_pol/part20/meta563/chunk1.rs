//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2122/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122(t10517: f64, t3103: f64, t1041: f64, t10868: f64, t248: f64, t2780: f64, t10316: f64, t3051: f64, t10277: f64, t976: f64, t10993: f64, t2960: f64) -> (f64, f64, f64, f64, f64) {
    let t42428 = t10517 * t3103;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42436 = t1041 * t248 * t3051 * t10316;
    let t42444 = t976 * t10277;
    let t42460 = t2960 * t10993;
    (t42428, t42432, t42436, t42444, t42460)
}
