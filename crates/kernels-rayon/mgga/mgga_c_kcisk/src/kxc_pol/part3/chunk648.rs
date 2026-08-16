//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 648/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk648(t10391: f64, t5182: f64, t1757: f64, t3293: f64, t5193: f64, t5192: f64, t1060: f64) -> (f64, f64, f64) {
    let t10392 = t5182 * t10391;
    let t10394 = t3293 * t1757;
    let t10395 = t5193 * t10394;
    let t10396 = t5192 * t10395;
    let t10397 = t5182 * t10396;
    let t10399 = t3293 * t1060;
    (t10392, t10397, t10399)
}
