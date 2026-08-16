//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1443/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1443(t2144: f64, t22298: f64, t103939: f64, t103943: f64, t103950: f64, t103954: f64, t11914: f64, t11915: f64, t1244: f64, t1246: f64, t1716: f64, t2147: f64, t21758: f64, t24776: f64, t27406: f64, t27460: f64, t29720: f64, t29726: f64, t29736: f64, t29777: f64, t491: f64, t5064: f64, t5971: f64, t6138: f64, t6146: f64, t6218: f64, t7283: f64, t7363: f64, t8054: f64, t8077: f64, t85909: f64, t95092: f64) -> (f64, f64) {
    let t109385 = t2144 * t22298;
    let t109393 = -0.24674011002723396548e-1_f64 * t7283 * t6146 * t8077 - 0.24674011002723396548e-1_f64 * t7283 * t1716 * t29726 + 0.10966227112321509577e-1_f64 * t7283 * t24776 * t27460 * t5971 - 0.29243272299524025538e-1_f64 * t27406 * t29777 - 0.8529287754027840782e-2_f64 * t7283 * t85909 * t7363 * t21758 + 0.43864908449286038307e-1_f64 * t95092 * t29736 + 3.0_f64 * t1244 * t8054 * t6218 * t1246 + 0.80418998823691070229e-1_f64 * t103939 - 0.24674011002723396548e-1_f64 * t7283 * t1716 * t2147 * t491 * t6138 + t11914 * t109385 * t11915 - 0.27415567780803773942e-2_f64 * t103943 + 6.0_f64 * t5064 * t29720 - 0.16449340668482264365e-1_f64 * t103950 - 0.54831135561607547883e-2_f64 * t103954;
    (t109385, t109393)
}
