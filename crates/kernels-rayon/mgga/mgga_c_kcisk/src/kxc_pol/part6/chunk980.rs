//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 980/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk980(t1341: f64, t30233: f64, t1340: f64, t12827: f64, t30158: f64, t442: f64) -> (f64, f64) {
    let t30234 = t1341 * t30233;
    let t30235 = t1340 * t30234;
    let t30236 = t12827 * t30235;
    let t30238 = t442 * t30158;
    (t30236, t30238)
}
