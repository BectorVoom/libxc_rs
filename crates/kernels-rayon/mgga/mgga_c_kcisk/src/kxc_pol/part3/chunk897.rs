//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 897/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk897(t13394: f64, t3776: f64, t1340: f64, t1411: f64, t10500: f64, t472: f64, t3913: f64, t470: f64, t1440: f64, t1415: f64, t382: f64, t1286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13395 = t3776 * t13394;
    let t13396 = t1340 * t13395;
    let t13397 = t1411 * t13396;
    let t13399 = t10500 * t472;
    let t13400 = 0.73697530864197530862e-3_f64 * t13399;
    let t13401 = t3913 * t470;
    let t13402 = t13401 * t1440;
    let t13403 = t1415 * t13402;
    let t13404 = t1411 * t13403;
    let t13406 = t3913 * t382;
    let t13407 = t13406 * t1286;
    (t13397, t13399, t13400, t13401, t13404, t13407)
}
