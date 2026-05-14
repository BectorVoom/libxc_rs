//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 857/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk857<F: Float>(t14115: F, t14149: F, t416: F, t467: F, t471: F, t415: F, t3508: F, t3733: F, t1411: F, t1220: F, t13441: F, t13466: F, t13470: F, t13935: F, t13940: F, t13947: F, t13952: F, t13956: F, t13960: F, t13962: F) -> (F, F, F, F) {
    let t14150 = t14115 + t14149;
    let t14151 = t416 * t14150;
    let t14152 = t14151 * t467;
    let t14153 = t14152 * t471;
    let t14154 = t415 * t14153;
    let t14156 = t3508 * t3733;
    let t14157 = t1411 * t14156;
    let t14159 = 0.82909722222222222219e-2 * t13466 - 0.8290972222222222222e-2 * t13470 - 0.193e0 * t1220 * t13935 + 0.2653111111111111111e-1 * t13940 - 0.386e0 * t1220 * t13441 + 0.1492375e-1 * t13947 - 0.49745833333333333332e-2 * t13952 - 0.11054629629629629629e-2 * t13956 + 0.44218518518518518516e-2 * t13960 + 0.49745833333333333332e-2 * t13962 + 0.24872916666666666666e-2 * t14154 - 0.74618749999999999998e-2 * t14157;
    (t14150, t14154, t14157, t14159)
}
