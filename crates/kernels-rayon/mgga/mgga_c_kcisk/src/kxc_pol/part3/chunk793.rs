//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 793/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk793(t12246: f64, t782: f64, t2009: f64, t5465: f64, t2005: f64, t5477: f64, t2019: f64, t657: f64, t2023: f64, t5509: f64, t1586: f64, t163: f64, t397: f64) -> (f64, f64, f64, f64, f64) {
    let t12248 = 0.9994882620098509563e-2_f64 * t782 * t12246;
    let t12249 = t5465 * t2009;
    let t12251 = t2005 * t5477;
    let t12253 = t2019 * t2019;
    let t12254 = 1.0_f64 / t12253;
    let t12255 = t657 * t12254;
    let t12256 = t5509 * t2023;
    let t12257 = t12255 * t12256;
    let t12258 = t1586 * t12257;
    let t12261 = t397 * t163;
    (t12248, t12249, t12251, t12258, t12261)
}
