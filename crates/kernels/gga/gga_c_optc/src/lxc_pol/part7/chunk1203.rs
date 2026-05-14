//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1203/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1203<F: Float>(t3067: F, t8693: F, t8751: F, t1102: F, t2917: F, t8749: F, t8791: F, t3012: F, t8688: F, t2994: F, t8686: F, t241: F, t8868: F, t1104: F, t8558: F, t8565: F) -> (F, F, F, F, F, F) {
    let t26457 = 0.2077890707925103596e3 * t3067 * t8693;
    let t26459 = 0.4155781415850207192e3 * t3067 * t8751;
    let t26463 = 0.62336721237753107879e3 * t1102 * t8749 * t2917 * t8791;
    let t26464 = t3012 * t8688;
    let t26467 = 0.3103500882342370105e4 * t8686 * t26464 * t2994;
    let t26468 = t241 * t8868;
    let t26470 = 0.23392893589820816284e1 * t26468 * t1104;
    let t26472 = 24.0 * t8558 * t8565;
    (t26457, t26459, t26463, t26467, t26470, t26472)
}
