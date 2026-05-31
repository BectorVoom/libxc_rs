//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 800/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk800<F: Float>(t1048: F, t7040: F, t795: F, t2266: F, t2267: F, t2526: F, t2271: F, t2810: F, t2813: F, t2452: F, t410: F, t372: F, t4845: F, t7025: F, t7028: F, t7031: F, t7033: F, t7036: F, t7039: F) -> (F, F) {
    let t7042 = t1048 * t7040 * t795;
    let t7043 = F::cast_from(2.0_f64) * t7042;
    let t7045 = t2266 * t2267 * t2526;
    let t7046 = F::cast_from(6.0_f64) * t7045;
    let t7048 = F::cast_from(0.4726e1_f64) * t2271 * t2810;
    let t7050 = F::cast_from(0.4726e1_f64) * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7052 = F::cast_from(8.0_f64) * t7051;
    let t7053 = t372 * t7028 + t4845 - t7025 - t7031 - t7033 + t7036 - t7039 + t7043 - t7046 - t7048 - t7050 + t7052;
    (t7052, t7053)
}
