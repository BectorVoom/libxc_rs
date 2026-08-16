//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2290/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290<F: Float>(t28525: F, t344: F, t6740: F, t5904: F, t6764: F, t1046: F, t17681: F, t17890: F, t23419: F, t23483: F, t23544: F, t28526: F, t28578: F, t28582: F, t28587: F, t5857: F, t5861: F, t6735: F, t6747: F, t6765: F, t83117: F, t83121: F, t88548: F) -> F {
    let t99720 = t6740 * t28525 * t344;
    let t99731 = t5904 * t6764;
    let t99736 = t23419 * t17681 / F::cast_from(2304.0_f64) - F::cast_from(0.80745512188280781712e-3_f64) * t23483 * t28587 - F::cast_from(0.16149102437656156342e-2_f64) * t83121 * t28578 + F::cast_from(0.80745512188280781712e-3_f64) * t83121 * t28582 + F::cast_from(0.10093189023535097714e-3_f64) * t99720 * t6747 + F::cast_from(0.20186378047070195428e-3_f64) * t83117 * t28578 + t23544 * t5857 / F::cast_from(2304.0_f64) + t6765 * t17890 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t23544 * t5861 + t99731 * t1046 / F::cast_from(2304.0_f64) - t88548 - F::cast_from(0.10093189023535097714e-3_f64) * t28526 * t6735;
    t99736
}
