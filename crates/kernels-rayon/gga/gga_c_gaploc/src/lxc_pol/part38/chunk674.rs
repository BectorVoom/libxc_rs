//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 674/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk674(t12411: f64, t295: f64, t10007: f64, t935: f64, t9438: f64, t825: f64, t10012: f64, t2684: f64, t2321: f64, t3371: f64, t882: f64, t3334: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12580 = t295 * t12411;
    let t12691 = t10007 * t935;
    let t12692 = t9438 * t12691;
    let t12693 = t825 * t12692;
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12770 = t3371 * t2321;
    let t12771 = t882 * t12770;
    let t12784 = t3334 * t871;
    (t12580, t12691, t12692, t12693, t12704, t12705, t12706, t12770, t12771, t12784)
}
