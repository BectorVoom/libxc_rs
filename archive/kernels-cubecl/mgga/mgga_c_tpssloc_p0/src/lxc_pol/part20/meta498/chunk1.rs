//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2006/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2006<F: Float>(t12521: F, t12524: F, t12813: F, t1401: F, t1458: F, t16506: F, t16521: F, t16524: F, t16535: F, t16538: F, t16541: F, t2319: F, t2363: F, t3938: F, t3941: F, t4072: F, t5371: F, t5376: F, t577: F, t671: F) -> F {
    let t16546 = F::cast_from(0.45e1_f64) * t16506 * t577 + F::cast_from(27.0_f64) * t16521 * t671 + F::cast_from(27.0_f64) * t16524 * t2319 + F::cast_from(0.135e2_f64) * t5371 * t2363 + F::cast_from(0.135e2_f64) * t12521 * t1458 + F::cast_from(54.0_f64) * t12524 * t5376 + F::cast_from(27.0_f64) * t3938 * t4072 + F::cast_from(27.0_f64) * t16535 * t1458 + F::cast_from(54.0_f64) * t3941 * t16538 + F::cast_from(27.0_f64) * t3941 * t16541 + F::cast_from(0.135e2_f64) * t1401 * t12813;
    t16546
}
