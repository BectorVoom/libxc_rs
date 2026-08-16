//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2037/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2037(t16080: f64, t16121: f64, t225: f64, t3856: f64, t5335: f64, t3851: f64, t5348: f64, t1332: f64, t1336: f64, t1381: f64, t16033: f64, t16037: f64, t16041: f64, t16044: f64, t16047: f64, t16049: f64, t16052: f64, t16055: f64, t16060: f64, t16065: f64, t16068: f64, t3777: f64, t3902: f64, t5234: f64, t5334: f64, t5336: f64, t5344: f64, t5345: f64, t5349: f64, t5351: f64, t564: f64) -> (f64, f64, f64, f64, f64) {
    let t16122 = t16080 + t16121;
    let t16123 = t16122 * t225;
    let t16125 = t5335 * t3856;
    let t16127 = t5348 * t3851;
    let t16131 = 2.0_f64 * t1332 * t5351 - t1336 * t16127 - 2.0_f64 * t1381 * t16060 - 2.0_f64 * t16033 * t5345 + 4.0_f64 * t16037 * t5334 + 4.0_f64 * t16041 * t5334 - t16044 * t5344 - 6.0_f64 * t16047 * t16049 + 6.0_f64 * t16052 * t5334 + 4.0_f64 * t16055 * t5336 + 2.0_f64 * t16065 * t5334 - 2.0_f64 * t16068 * t5344 + t16123 * t564 - t16125 * t5344 - 2.0_f64 * t3777 * t5349 - 2.0_f64 * t3902 * t5234;
    (t16122, t16123, t16125, t16127, t16131)
}
