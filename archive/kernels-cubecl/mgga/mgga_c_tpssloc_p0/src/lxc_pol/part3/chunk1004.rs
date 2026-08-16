//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1004/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1004<F: Float>(t12997: F, t792: F, t12984: F, t686: F, t776: F, t12986: F, t12990: F, t12994: F, t4127: F, t9526: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F) -> F {
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = F::cast_from(0.49999999999999999998e-2_f64) * t12998 * t13000;
    let t13003 = F::cast_from(0.33333333333333333332e-2_f64) * t9526 - t9540 - F::cast_from(0.25925925925925925926e-1_f64) * t9542 + F::cast_from(0.38888888888888888888e-2_f64) * t9544 - F::cast_from(0.10555555555555555555e-1_f64) * t9547 - F::cast_from(0.25e-2_f64) * t9552 + F::cast_from(0.83333333333333333332e-3_f64) * t9556 + F::cast_from(0.16666666666666666666e-2_f64) * t12986 + F::cast_from(0.99999999999999999996e-2_f64) * t4127 * t12990 + F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t12994 - t13002;
    t13003
}
