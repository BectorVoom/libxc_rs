//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2037/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2037<F: Float>(t2098: F, t5381: F, t27286: F, t576: F, t112: F, t27240: F, t12521: F, t12524: F, t1401: F, t1458: F, t16521: F, t16524: F, t2039: F, t2363: F, t23917: F, t24462: F, t24478: F, t24481: F, t27170: F, t27254: F, t27273: F, t27276: F, t3941: F, t4072: F, t5371: F, t5376: F, t55353: F, t55405: F, t671: F, t7056: F, t7235: F, t7801: F, t84033: F, t84078: F, t92128: F) -> (F, F, F) {
    let t94120 = F::cast_from(2.0_f64) * t2098 * t5381;
    let t94122 = F::cast_from(2.0_f64) * t576 * t27286;
    let t94127 = t27240 * t112;
    let t94160 = F::cast_from(0.135e2_f64) * t84078 * t1458 + F::cast_from(27.0_f64) * t55405 * t2039 + F::cast_from(27.0_f64) * t94127 * t671 + F::cast_from(54.0_f64) * t12524 * t27273 + F::cast_from(54.0_f64) * t12524 * t27276 + F::cast_from(27.0_f64) * t24462 * t4072 + F::cast_from(0.135e2_f64) * t27254 * t2363 + F::cast_from(0.135e2_f64) * t12521 * t7801 + F::cast_from(54.0_f64) * t84033 * t5376 + F::cast_from(54.0_f64) * t3941 * t27170 * t671 + F::cast_from(27.0_f64) * t3941 * t7801 * t2363 + F::cast_from(54.0_f64) * t55353 * t7235 + F::cast_from(54.0_f64) * t16524 * t24478 + F::cast_from(27.0_f64) * t16521 * t7056 + F::cast_from(0.135e2_f64) * t5371 * t23917 + F::cast_from(0.135e2_f64) * t1401 * t92128 + F::cast_from(27.0_f64) * t16524 * t24481;
    (t94120, t94122, t94160)
}
