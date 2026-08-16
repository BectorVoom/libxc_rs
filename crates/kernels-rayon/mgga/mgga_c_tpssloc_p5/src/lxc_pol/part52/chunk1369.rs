//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1369/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1369(t12571: f64, t31863: f64, t116114: f64, t39063: f64, t45844: f64, t8662: f64, t33676: f64, t9239: f64, t116082: f64, t116111: f64, t116115: f64, t116119: f64, t116124: f64, t119913: f64, t119938: f64, t119944: f64, t119952: f64, t121024: f64, t121032: f64, t121074: f64, t121081: f64, t121087: f64, t31677: f64, t31684: f64, t31693: f64, t31857: f64, t31860: f64, t31868: f64, t33564: f64, t33568: f64, t33572: f64, t33669: f64, t33677: f64, t8663: f64) -> f64 {
    let t122976 = t12571 * t31863;
    let t122979 = t39063 * t116114;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123020 = -5.0_f64 / 36.0_f64 * t122976 * t31684 + 35.0_f64 / 24.0_f64 * t122979 * t121024 - 5.0_f64 / 12.0_f64 * t116115 * t121032 - 5.0_f64 / 36.0_f64 * t116111 * t33568 - 5.0_f64 / 36.0_f64 * t116119 * t33568 - 5.0_f64 / 24.0_f64 * t122988 * t31677 + 5.0_f64 / 72.0_f64 * t33669 * t31693 - 5.0_f64 / 24.0_f64 * t116124 * t33564 - 5.0_f64 / 24.0_f64 * t116082 * t33564 - 5.0_f64 / 24.0_f64 * t31860 * t119913 - 5.0_f64 / 24.0_f64 * t31860 * t121074 - 5.0_f64 / 24.0_f64 * t123001 * t31677 + 5.0_f64 / 72.0_f64 * t33677 * t31693 + 5.0_f64 / 72.0_f64 * t31857 * t33572 + 5.0_f64 / 72.0_f64 * t31868 * t33572 + 5.0_f64 / 72.0_f64 * t8663 * t121081 + 5.0_f64 / 72.0_f64 * t8663 * t119952 + 5.0_f64 / 72.0_f64 * t8663 * t121087 - 5.0_f64 / 24.0_f64 * t31860 * t119938 + 5.0_f64 / 72.0_f64 * t8663 * t119944;
    t123020
}
