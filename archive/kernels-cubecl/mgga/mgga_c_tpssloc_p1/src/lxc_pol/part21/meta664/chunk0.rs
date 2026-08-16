//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2465/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2465<F: Float>(t1174: F, t3442: F, t44571: F, t11588: F, t3475: F, t1176: F, t697: F, t1184: F, t3447: F, t3451: F, t11153: F, t460: F) -> (F, F, F, F, F, F) {
    let t44573 = t1174 * t44571 * t3442;
    let t44579 = t11588 * t3475;
    let t44583 = t697 * t1176;
    let t44584 = t44583 * t1184;
    let t44586 = t3447 * t44584 * t3451;
    let t44607 = t460 * t11153;
    (t44573, t44579, t44583, t44584, t44586, t44607)
}
