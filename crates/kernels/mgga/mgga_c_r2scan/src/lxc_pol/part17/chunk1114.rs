//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1114/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1114<F: Float>(t3269: F, t45081: F, t12739: F, t42916: F, t10610: F, t11199: F, t12414: F, t12056: F, t3275: F, t7040: F, t8601: F, t12029: F, t40713: F, t12042: F, t38339: F, t38356: F, t38359: F, t39122: F, t39127: F, t39129: F, t39130: F, t39131: F, t39134: F) -> (F, F, F, F, F, F, F, F) {
    let t45083 = 45.0 / 64.0 * t3269 * t45081;
    let t45085 = 3.0 / 2.0 * t42916 * t12739;
    let t45088 = 3.0 / 2.0 * t10610 * t11199 * t12414;
    let t45094 = t3275 * t12056 * t7040 / 2.0;
    let t45097 = t3275 * t11199 * t8601 / 4.0;
    let t45099 = 5.0 / 8.0 * t40713 * t12029;
    let t45100 = t40713 * t12042;
    let t45101 = -t45083 - t45085 - t45088 + t39122 + t39127 + 0.162600798888400151e-2 * t38339 - t39129 + t39130 - t39131 - 0.38422568777328955681e-2 * t38356 + 0.60975299583150056624e-3 * t38359 + t39134 + t45094 + t45097 - t45099 - t45100;
    (t45083, t45085, t45088, t45094, t45097, t45099, t45100, t45101)
}
