//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 921/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk921(t1248: f64, t12831: f64, t13607: f64, t1249: f64, t12925: f64, t398: f64, t963: f64, t1163: f64, t13522: f64, t13526: f64, t13530: f64, t13533: f64, t13536: f64, t13540: f64, t13543: f64, t13546: f64, t13549: f64, t13552: f64, t13555: f64) -> (f64, f64, f64, f64, f64) {
    let t13609 = t1248 * t13607 * t12831;
    let t13612 = t1248 * t1249 * t12925;
    let t13614 = t963 * t398;
    let t13616 = t1248 * t13614 * t1163;
    let t13618 = 28.0_f64 / 27.0_f64 * t13522;
    let t13629 = -t13618 - 4.0_f64 / 9.0_f64 * t13526 + 2.0_f64 / 9.0_f64 * t13530 - 2.0_f64 / 3.0_f64 * t13533 + t13536 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t13540 + 4.0_f64 / 3.0_f64 * t13543 - 2.0_f64 / 3.0_f64 * t13546 - 2.0_f64 * t13549 + 2.0_f64 * t13552 - t13555 / 3.0_f64;
    (t13609, t13612, t13614, t13616, t13629)
}
