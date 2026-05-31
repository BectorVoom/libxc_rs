//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1287/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1287<F: Float>(t11199: F, t3275: F, t8601: F, t12029: F, t40713: F, t12042: F, t38339: F, t38356: F, t38359: F, t39122: F, t39127: F, t39129: F, t39130: F, t39131: F, t39134: F, t45083: F, t45085: F, t45088: F, t45094: F) -> (F, F, F, F) {
    let t45097 = t3275 * t11199 * t8601 / F::cast_from(4.0_f64);
    let t45099 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40713 * t12029;
    let t45100 = t40713 * t12042;
    let t45101 = -t45083 - t45085 - t45088 + t39122 + t39127 + F::cast_from(0.162600798888400151e-2_f64) * t38339 - t39129 + t39130 - t39131 - F::cast_from(0.38422568777328955681e-2_f64) * t38356 + F::cast_from(0.60975299583150056624e-3_f64) * t38359 + t39134 + t45094 + t45097 - t45099 - t45100;
    (t45097, t45099, t45100, t45101)
}
