//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1050/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1050<F: Float>(t21044: F, t4230: F, t19928: F, t4204: F, t4203: F, t14364: F, t469: F, t19917: F, t6332: F, t21022: F, t21025: F, t21027: F, t21031: F, t21033: F, t21036: F, t21039: F, t21042: F) -> (F, F, F, F, F, F) {
    let t21045 = t4230 * t21044;
    let t21047 = t4204 * t19928;
    let t21048 = t4203 * t21047;
    let t21050 = t14364 * t469;
    let t21051 = t6332 * t19917;
    let t21052 = t21050 * t21051;
    let t21054 = 2.0 / 9.0 * t21022 + t21025 / 96.0 - t21027 / 8.0 + t21031 / 864.0 - t21033 / 18.0 + t21036 / 54.0 - t21039 / 48.0 - 19.0 / 108.0 * t21042 + t21045 / 576.0 - t21048 / 24.0 - 3.0 / 8.0 * t21052;
    (t21045, t21047, t21048, t21051, t21052, t21054)
}
