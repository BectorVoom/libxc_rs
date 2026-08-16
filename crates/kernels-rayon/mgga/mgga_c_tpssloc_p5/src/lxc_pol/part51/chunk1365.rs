//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1365/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1365(t115876: f64, t33564: f64, t31688: f64, t33572: f64, t115837: f64, t115846: f64, t115853: f64, t115860: f64, t115866: f64, t115877: f64, t115891: f64, t115894: f64, t115895: f64, t119888: f64, t119897: f64, t119913: f64, t119931: f64, t121024: f64, t121029: f64, t121032: f64, t121040: f64, t121044: f64, t121050: f64, t121055: f64, t121058: f64, t2240: f64, t31675: f64, t31681: f64, t31684: f64, t33568: f64, t39063: f64, t63: f64) -> f64 {
    let t121064 = t115876 * t33564;
    let t121066 = t31688 * t33572;
    let t121072 = 10.0_f64 / 27.0_f64 * t115837 - 35.0_f64 / 12.0_f64 * t39063 * t115894 * t121024 + 5.0_f64 / 18.0_f64 * t31681 * t119897 - 20.0_f64 / 27.0_f64 * t121029 + 5.0_f64 / 6.0_f64 * t115895 * t121032 + 5.0_f64 / 18.0_f64 * t115891 * t33568 + 5.0_f64 / 18.0_f64 * t31681 * t119888 + 5.0_f64 / 18.0_f64 * t31681 * t121040 + 5.0_f64 / 18.0_f64 * t31681 * t121044 - 5.0_f64 / 9.0_f64 * t2240 * t119931 * t63 * t121050 + 5.0_f64 / 6.0_f64 * t115895 * t121055 + 5.0_f64 / 18.0_f64 * t121058 * t31684 + 5.0_f64 / 27.0_f64 * t115846 + 5.0_f64 / 27.0_f64 * t115853 - t115860 - 10.0_f64 / 9.0_f64 * t115877 - 10.0_f64 / 9.0_f64 * t121064 + 10.0_f64 / 27.0_f64 * t121066 + 5.0_f64 / 12.0_f64 * t115866 * t33564 + 5.0_f64 / 12.0_f64 * t31675 * t119913;
    t121072
}
