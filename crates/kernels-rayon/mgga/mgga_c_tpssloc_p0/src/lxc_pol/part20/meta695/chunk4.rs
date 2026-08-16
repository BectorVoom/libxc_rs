//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2651/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651(t1351: f64, t3850: f64, t12290: f64, t5234: f64, t16060: f64, t3789: f64, t12012: f64, t12215: f64, t12293: f64, t12303: f64, t12420: f64, t16048: f64, t16224: f64, t16233: f64, t16235: f64, t16242: f64, t16306: f64, t1810: f64, t1825: f64, t210: f64, t3719: f64, t3733: f64, t3734: f64, t3795: f64, t3803: f64, t39971: f64, t5226: f64, t5248: f64, t53985: f64, t53990: f64, t53998: f64, t54003: f64, t54013: f64, t54014: f64) -> (f64, f64) {
    let t54015 = t1351 * t3850;
    let t54020 = t5234 * t12290;
    let t54023 = t16060 * t3789;
    let t54026 = -5.0_f64 / 256.0_f64 * t3803 * t16224 * t1825 * t12303 - 7.0_f64 / 384.0_f64 * t39971 - t53985 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t16306 * t12420 - 3.0_f64 / 512.0_f64 * t53990 * t16235 - 3.0_f64 / 512.0_f64 * t16233 * t5248 * t16242 * t16048 + t53998 - 3.0_f64 / 4.0_f64 * t12215 * t210 * t5226 * t3734 - 7.0_f64 / 16.0_f64 * t54003 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t5226 * t3719 + t3733 * t210 * t1810 * t12012 / 16.0_f64 - 3.0_f64 / 512.0_f64 * t16233 * t54013 * t54014 * t54015 - t54020 * t12293 / 512.0_f64 + t54023 * t3795 / 512.0_f64;
    (t54015, t54026)
}
