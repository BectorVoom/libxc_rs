//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1841/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1841<F: Float>(t13804: F, t13845: F, t13894: F, t13937: F, t225: F, t68: F, t369: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F) -> (F, F, F, F, F, F, F) {
    let t13939 = t13804 + t13845 + t13894 + t13937;
    let t13940 = t13939 * t225;
    let t13941 = t13940 * t68;
    let t13942 = t13941 * t369;
    let t13946 = t4622 * t1036 / F::cast_from(432.0_f64);
    let t13948 = t3117 * t4571 / F::cast_from(3456.0_f64);
    let t13950 = t248 * t3051 * t4347;
    (t13939, t13940, t13941, t13942, t13946, t13948, t13950)
}
