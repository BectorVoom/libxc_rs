//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2200/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2200<F: Float>(t107: F, t9576: F, t2585: F, t667: F, t2281: F, t2333: F, t2359: F, t626: F, t9367: F, t9371: F, t9412: F, t106: F, t9364: F) -> (F, F, F, F, F, F, F, F) {
    let t45421 = F::cast_from(2618.0_f64) / F::cast_from(81.0_f64) * t9576 * t107;
    let t45422 = t2585 * t667;
    let t45424 = t2281 * t2333;
    let t45426 = t2281 * t2359;
    let t45428 = t626 * t9367;
    let t45430 = t626 * t9371;
    let t45432 = t626 * t9412;
    let t45435 = F::cast_from(1.0_f64) / t9364 / t106;
    (t45421, t45422, t45424, t45426, t45428, t45430, t45432, t45435)
}
