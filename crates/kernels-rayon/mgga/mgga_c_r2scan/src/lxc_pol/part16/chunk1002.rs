//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1002/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1002(t12422: f64, t3271: f64, t10924: f64, t10933: f64, t11589: f64, t11593: f64, t11604: f64, t12406: f64, t12410: f64, t12413: f64, t12417: f64, t12420: f64) -> (f64, f64) {
    let t12423 = t12422 * t3271;
    let t12424 = t12423 / 4.0_f64;
    let t12425 = 0.72042316457491791906e-3_f64 * t11589 - 0.10248087766267884742e-3_f64 * t11593 + t12406 - 0.30487649791575028314e-3_f64 * t11604 - t12410 + t12413 - t12417 - t12420 + t10924 - t10933 - t12424;
    (t12424, t12425)
}
