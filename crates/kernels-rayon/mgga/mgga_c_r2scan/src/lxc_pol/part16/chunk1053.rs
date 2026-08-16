//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1053/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1053(t166: f64, t874: f64, t37426: f64, t37427: f64, t424: f64, t10645: f64, t10976: f64, t2104: f64, t3437: f64, t58: f64, t10929: f64, t3428: f64, t3430: f64, t6826: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37428 = t166 * t874;
    let t37431 = t37426 * t37427 * t424 * t37428;
    let t37434 = t10645 * t10976 * t2104;
    let t37435 = t3437 * t58;
    let t37438 = t37434 * t37435 * t424 * t10929;
    let t37442 = t6826 * t761 * t3428 * t3430;
    (t37428, t37431, t37434, t37435, t37438, t37442)
}
