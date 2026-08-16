//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2730/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730(t12250: f64, t5286: f64, t12240: f64, t12267: f64, t1336: f64, t1351: f64, t16033: f64, t16044: f64, t16047: f64, t16055: f64, t16206: f64, t19660: f64, t19668: f64, t19732: f64, t19739: f64, t19745: f64, t19748: f64, t19752: f64, t19810: f64, t20018: f64, t3777: f64, t3851: f64, t3901: f64, t3909: f64, t5334: f64, t5335: f64, t5344: f64, t54976: f64, t6378: f64, t6448: f64) -> f64 {
    let t57568 = t12250 * t5286;
    let t57597 = -24.0_f64 * t1351 * t16047 * t5335 * t57568 + 4.0_f64 * t12240 * t19739 * t5334 - 2.0_f64 * t1336 * t19732 * t3901 - 2.0_f64 * t16206 * t5335 * t5344 - t19660 * t3851 * t5344 + 2.0_f64 * t12267 * t6448 - 4.0_f64 * t16033 * t20018 - 2.0_f64 * t16044 * t19810 + 12.0_f64 * t16055 * t19748 + 4.0_f64 * t19668 * t3777 - 12.0_f64 * t19745 * t54976 - 4.0_f64 * t19752 * t3777 + t3909 * t6378;
    t57597
}
