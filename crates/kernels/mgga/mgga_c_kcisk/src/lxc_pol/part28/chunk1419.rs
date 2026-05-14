//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1419/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1419<F: Float>(t4597: F, t5439: F, t122506: F, t34560: F, t34389: F, t34416: F, t123: F, t2801: F, t35437: F, t112904: F, t2023: F, t7261: F, t9162: F, t35409: F, t9724: F, t10000: F, t10014: F, t112867: F, t117621: F, t118120: F, t24934: F, t33176: F, t33196: F, t34400: F, t34406: F, t34411: F, t34452: F, t34457: F, t34520: F, t34561: F, t35416: F, t9721: F, t9728: F, t9740: F, t9743: F) -> (F, F, F) {
    let t122552 = t5439 * t4597;
    let t122554 = t34560 * t122552 * t122506;
    let t122561 = t34416 * t34389;
    let t122564 = t2801 * t35437 * t123;
    let t122573 = t7261 * t112904 * t9162 * t2023;
    let t122583 = t9724 * t35409;
    let t122594 = 0.23148148148148148148e-2 * t9740 * t122554 + 0.23148148148148148148e-2 * t9740 * t112867 * t34561 * t24934 - 0.11574074074074074074e-2 * t122561 - 0.16975308641975308642e-1 * t122564 * t9743 + 0.10722222222222222222e-1 * t118120 * t34400 + 0.32166666666666666666e-1 * t118120 * t34406 + 0.60312500000000000001e-2 * t33196 * t122573 + 0.62081666666666666665e-2 * t33176 * t34411 * t34406 - 0.120625e-1 * t117621 * t34406 + 0.10416666666666666667e-1 * t9740 * t122573 - 0.10722222222222222222e-1 * t122583 * t9728 + 0.10416666666666666667e-1 * t34452 * t10014 + 0.10416666666666666667e-1 * t34457 * t10014 + 0.10416666666666666667e-1 * t10000 * t34520 - 0.10416666666666666667e-1 * t9721 * t35416;
    (t122554, t122573, t122594)
}
