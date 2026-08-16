//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 755/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk755<F: Float>(t109: F, t16535: F, t1873: F, t6534: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2039: F, t3652: F, t22468: F, t22471: F, t22474: F, t22476: F) -> (F, F, F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t23892 = F::cast_from(27.0_f64) * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = F::cast_from(54.0_f64) * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = F::cast_from(27.0_f64) * t3941 * t23896;
    let t23900 = F::cast_from(0.135e2_f64) * t1401 * t22479;
    let t23909 = t3652 * t2039;
    let t23912 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t23917 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t23912 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t22471 + t22474 / F::cast_from(2.0_f64) - t22476 / F::cast_from(4.0_f64));
    (t23892, t23893, t23895, t23896, t23898, t23900, t23909, t23917)
}
