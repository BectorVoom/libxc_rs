//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1067/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1067<F: Float>(t26717: F, t8030: F, t26854: F, t1014: F, t27931: F, t27964: F, t7699: F, t27851: F, t1009: F, t14400: F, t8048: F, t9562: F, t27925: F, t27882: F, t26748: F, t27803: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t96010 = 0.46336805555555555556e-3 * t8030 * t26717;
    let t96015 = t8030 * t26854;
    let t96018 = t1014 * t27931;
    let t96019 = 0.33163888888888888888e-2 * t96018;
    let t96026 = 0.12356481481481481482e-2 * t27964 * t7699;
    let t96068 = t1014 * t27851;
    let t96108 = t14400 * t1009;
    let t96121 = t9562 * t8048;
    let t96123 = t1014 * t27925;
    let t96124 = 0.33163888888888888888e-2 * t96123;
    let t96137 = t1014 * t27882;
    let t96138 = 0.33163888888888888888e-2 * t96137;
    let t96148 = 0.15445601851851851852e-3 * t26748 * t27803;
    (t96010, t96015, t96018, t96019, t96026, t96068, t96108, t96121, t96123, t96124, t96137, t96138, t96148)
}
