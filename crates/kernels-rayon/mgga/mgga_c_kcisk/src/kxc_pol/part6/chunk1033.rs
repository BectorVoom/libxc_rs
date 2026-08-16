//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1033/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1033(t5606: f64, t8093: f64, t3759: f64, t1341: f64, t30298: f64, t1340: f64, t1339: f64, t2231: f64, t8010: f64, t3776: f64, t1415: f64, t1411: f64) -> (f64, f64, f64, f64) {
    let t30955 = t5606 * t8093;
    let t30956 = t3759 * t30955;
    let t30958 = t1341 * t30298;
    let t30959 = t1340 * t30958;
    let t30960 = t1339 * t30959;
    let t30962 = t2231 * t8010;
    let t30963 = t3776 * t30962;
    let t30964 = t1415 * t30963;
    let t30965 = t1411 * t30964;
    (t30956, t30960, t30962, t30965)
}
