//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1008/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1008(t31682: f64, t5398: f64, t8308: f64, t113875: f64, t121022: f64, t1433: f64, t126103: f64, t1862: f64, t8513: f64, t115860: f64, t115895: f64, t121029: f64, t121058: f64, t121064: f64, t121066: f64, t126070: f64, t126100: f64, t31681: f64, t33115: f64, t33560: f64, t33568: f64, t55921: f64, t8511: f64, t8512: f64, t8515: f64) -> f64 {
    let t128311 = t8308 * t31682 * t5398;
    let t128317 = t113875 * t121022 * t1433;
    let t128326 = t8513 * t126103 * t1862;
    let t128333 = -40.0_f64 / 27.0_f64 * t121029 + 5.0_f64 / 9.0_f64 * t31681 * t126070 + 5.0_f64 / 18.0_f64 * t31681 * t128311 + 5.0_f64 / 9.0_f64 * t121058 * t33568 + 5.0_f64 / 3.0_f64 * t115895 * t128317 - 5.0_f64 / 72.0_f64 * t55921 * t8511 * t8515 - 5.0_f64 / 36.0_f64 * t33560 * t33115 - t115860 - 5.0_f64 / 36.0_f64 * t8512 * t128326 - 5.0_f64 / 72.0_f64 * t8512 * t126100 - 20.0_f64 / 9.0_f64 * t121064 + 20.0_f64 / 27.0_f64 * t121066;
    t128333
}
