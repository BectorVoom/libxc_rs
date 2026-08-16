//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 938/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk938<F: Float>(t154: F, t3584: F, t3241: F, t636: F, t52: F, t1098: F, t3256: F, t1094: F, t3312: F, t3311: F, t419: F, t409: F) -> (F, F, F, F, F, F) {
    let t11145 = t154 * t3584;
    let t11147 = F::cast_from(1.0_f64) / t3241 / t636;
    let t11152 = t3241 * t52;
    let t11153 = F::cast_from(1.0_f64) / t11152;
    let t11180 = t3256 * t1098;
    let t11185 = t1094 * t3312;
    let t11189 = F::cast_from(1.0_f64) / t3311 / t419;
    let t11190 = t409 * t11189;
    (t11145, t11147, t11153, t11180, t11185, t11190)
}
