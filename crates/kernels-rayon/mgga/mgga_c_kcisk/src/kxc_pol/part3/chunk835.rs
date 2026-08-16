//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 835/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk835(t12692: f64, t12813: f64, t1010: f64, t10330: f64, t10333: f64, t10339: f64, t10342: f64, t10346: f64, t10348: f64, t10351: f64, t10353: f64, t10355: f64, t12404: f64, t12406: f64) -> (f64, f64) {
    let t12814 = t12692 + t12813;
    let t12815 = t1010 * t12814;
    let t12816 = t10330 - t10333 - t10339 + t10342 + t10346 - t10348 + t10351 + t10353 + t10355 - t12404 + t12406 - t12815;
    (t12815, t12816)
}
