//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1443/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1443<F: Float>(t2144: F, t22298: F, t103939: F, t103943: F, t103950: F, t103954: F, t11914: F, t11915: F, t1244: F, t1246: F, t1716: F, t2147: F, t21758: F, t24776: F, t27406: F, t27460: F, t29720: F, t29726: F, t29736: F, t29777: F, t491: F, t5064: F, t5971: F, t6138: F, t6146: F, t6218: F, t7283: F, t7363: F, t8054: F, t8077: F, t85909: F, t95092: F) -> (F, F) {
    let t109385 = t2144 * t22298;
    let t109393 = -F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t6146 * t8077 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t1716 * t29726 + F::cast_from(0.10966227112321509577e-1_f64) * t7283 * t24776 * t27460 * t5971 - F::cast_from(0.29243272299524025538e-1_f64) * t27406 * t29777 - F::cast_from(0.8529287754027840782e-2_f64) * t7283 * t85909 * t7363 * t21758 + F::cast_from(0.43864908449286038307e-1_f64) * t95092 * t29736 + F::cast_from(3.0_f64) * t1244 * t8054 * t6218 * t1246 + F::cast_from(0.80418998823691070229e-1_f64) * t103939 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t1716 * t2147 * t491 * t6138 + t11914 * t109385 * t11915 - F::cast_from(0.27415567780803773942e-2_f64) * t103943 + F::cast_from(6.0_f64) * t5064 * t29720 - F::cast_from(0.16449340668482264365e-1_f64) * t103950 - F::cast_from(0.54831135561607547883e-2_f64) * t103954;
    (t109385, t109393)
}
