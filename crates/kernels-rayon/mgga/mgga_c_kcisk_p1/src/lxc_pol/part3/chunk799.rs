//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 799/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk799(t11956: f64, t11981: f64, t12156: f64, t12338: f64, t2041: f64, t5525: f64, t2038: f64, t5531: f64, t2040: f64, t798: f64, t2049: f64, t5533: f64) -> (f64, f64, f64, f64, f64) {
    let t12340 = t11956 + t11981 + t12156 + t12338;
    let t12342 = t5525 * t2041;
    let t12345 = t2038 * t5531;
    let t12350 = t2040 * t2040;
    let t12351 = 1.0_f64 / t12350;
    let t12352 = t798 * t12351;
    let t12353 = t5533 * t2049;
    (t12340, t12342, t12345, t12352, t12353)
}
