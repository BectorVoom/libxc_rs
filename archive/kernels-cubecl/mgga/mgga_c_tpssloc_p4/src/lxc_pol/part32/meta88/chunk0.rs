//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 563/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk563<F: Float>(t25: F, t28: F, t17: F, t1788: F, t1787: F, t182: F, t1298: F, t1408: F, t1302: F, t1649: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1789 = t17 * t1788;
    let t1791 = F::cast_from(0.19751673498613801407e-1_f64) * t1787 * t182;
    let t1794 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1298 * t1408);
    let t1797 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1302 * t1649);
    let t1799 = t1794 / F::cast_from(2.0_f64) + t1797 / F::cast_from(2.0_f64);
    (t1789, t1791, t1799)
}
