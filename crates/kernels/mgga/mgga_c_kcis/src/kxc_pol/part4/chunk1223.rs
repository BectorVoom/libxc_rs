//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1223/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1223<F: Float>(t12217: F, t617: F, t16055: F, t16905: F, t16065: F, t1928: F, t610: F, t990: F, t4455: F, t6183: F, t1610: F, t6176: F, t4463: F, t6177: F, t4426: F, t6141: F) -> (F, F, F, F, F, F) {
    let t18183 = t12217 * t617;
    let t18184 = t18183 * t16055;
    let t18187 = t16905 * t617;
    let t18188 = t18187 * t16065;
    let t18192 = t610 * t1928 * t990;
    let t18195 = t4455 * t6183;
    let t18196 = t18195 * t1610;
    let t18197 = t6176 * t18196;
    let t18200 = t6177 * t4463;
    let t18201 = t6176 * t18200;
    let t18205 = t6141 * t4426 / 324.0;
    (t18184, t18188, t18192, t18197, t18201, t18205)
}
