//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1044/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1044(t113875: f64, t115896: f64, t641: f64, t1862: f64, t113876: f64, t31680: f64, t9239: f64, t113864: f64, t115833: f64, t113871: f64, t115863: f64, t115866: f64, t115871: f64, t115873: f64, t115877: f64, t115880: f64, t115884: f64, t115889: f64, t115891: f64, t115895: f64, t31672: f64, t31677: f64, t31681: f64, t31684: f64, t31693: f64, t7026: f64, t8512: f64) -> f64 {
    let t115898 = t113875 * t115896 * t641;
    let t115903 = t113875 * t1862;
    let t115904 = t115903 * t113876;
    let t115907 = t9239 * t31680;
    let t115908 = t115833 * t113864;
    let t115911 = -5.0_f64 / 36.0_f64 * t8512 * t115863 + 5.0_f64 / 6.0_f64 * t115866 * t31677 - 5.0_f64 / 18.0_f64 * t31672 * t31693 - 35.0_f64 / 12.0_f64 * t115871 * t115873 - 20.0_f64 / 9.0_f64 * t115877 + 5.0_f64 / 18.0_f64 * t7026 * t115880 + 5.0_f64 / 18.0_f64 * t31681 * t115884 - 40.0_f64 / 27.0_f64 * t115889 + 5.0_f64 / 9.0_f64 * t115891 * t31684 + 5.0_f64 / 3.0_f64 * t115895 * t115898 + 5.0_f64 / 9.0_f64 * t31681 * t113871 + 10.0_f64 / 9.0_f64 * t31681 * t115904 - 10.0_f64 / 3.0_f64 * t115907 * t115908;
    t115911
}
