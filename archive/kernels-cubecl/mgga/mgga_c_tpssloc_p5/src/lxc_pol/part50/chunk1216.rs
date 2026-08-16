//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1216/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1216<F: Float>(t113082: F, t118413: F, t118454: F, t118466: F, t118953: F, t16596: F, t1877: F, t23290: F, t23295: F, t2522: F, t25353: F, t25365: F, t25374: F, t30770: F, t4119: F, t4255: F, t4303: F, t4314: F, t6670: F, t7540: F, t8370: F) -> F {
    let t119676 = -F::cast_from(6.0_f64) * t113082 * t1877 * t25374 + F::cast_from(4.0_f64) * t118413 * t1877 * t23295 - F::cast_from(6.0_f64) * t118454 * t2522 * t6670 - F::cast_from(6.0_f64) * t118466 * t2522 * t6670 + F::cast_from(4.0_f64) * t118953 * t1877 * t23295 + F::cast_from(6.0_f64) * t16596 * t2522 * t30770 - F::cast_from(2.0_f64) * t1877 * t23290 * t7540 - F::cast_from(2.0_f64) * t1877 * t25353 * t6670 + F::cast_from(2.0_f64) * t1877 * t30770 * t4303 + F::cast_from(6.0_f64) * t2522 * t25365 * t30770 - F::cast_from(3.0_f64) * t2522 * t4119 * t8370 - F::cast_from(6.0_f64) * t4255 * t4314 * t8370;
    t119676
}
