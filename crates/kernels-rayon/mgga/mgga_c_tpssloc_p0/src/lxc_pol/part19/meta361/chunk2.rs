//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1311/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1311(t10397: f64, t10422: f64, t3070: f64, t3120: f64, t10517: f64, t3103: f64, t1041: f64, t10868: f64, t248: f64, t2780: f64, t10316: f64, t3051: f64) -> (f64, f64, f64, f64, f64) {
    let t42412 = t3070 * t10422 * t10397;
    let t42422 = t3120 * t3120;
    let t42428 = t10517 * t3103;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42436 = t1041 * t248 * t3051 * t10316;
    (t42412, t42422, t42428, t42432, t42436)
}
