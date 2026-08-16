//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3168/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168<F: Float>(t11539: F, t1174: F, t18211: F, t3540: F, t6170: F, t19015: F, t3577: F, t45124: F, t6158: F, t15730: F, t5002: F, t1226: F, t18573: F) -> (F, F, F, F, F, F) {
    let t65567 = t1174 * t11539 * t18211;
    let t65581 = t6170 * t3540;
    let t65598 = t3577 * t45124 * t19015;
    let t65600 = t6158 * t3540;
    let t65605 = t5002 * t15730;
    let t65607 = t18573 * t1226;
    (t65567, t65581, t65598, t65600, t65605, t65607)
}
