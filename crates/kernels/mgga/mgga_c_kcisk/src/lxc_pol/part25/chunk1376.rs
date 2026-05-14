//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1376/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1376<F: Float>(t34457: F, t9736: F, t33297: F, t34389: F, t33208: F, t17163: F, t34394: F, t9740: F, t112541: F, t112765: F, t116895: F, t116900: F, t117674: F, t33204: F, t33240: F, t34400: F, t34406: F, t34416: F, t34429: F, t34435: F) -> (F,) {
    let t118237 = 0.34722222222222222222e-2 * t34457 * t9736;
    let t118246 = 0.11574074074074074074e-2 * t33297 * t34389;
    let t118248 = 0.11574074074074074074e-2 * t33208 * t34389;
    let t118250 = t9740 * t17163 * t34394;
    let t118256 = -0.23148148148148148148e-2 * t34435 * t33204 - 0.120625e-1 * t112765 * t34406 - 0.10416666666666666667e-1 * t33297 * t34400 - t118237 + 0.61905925925925925924e-2 * t116895 + 0.11607361111111111111e-2 * t116900 + 0.77382407407407407407e-3 * t112541 + 0.31250000000000000001e-1 * t9740 * t117674 + 0.34722222222222222222e-2 * t34416 * t33240 - t118246 - t118248 - 0.81018518518518518518e-2 * t118250 - 0.10416666666666666667e-1 * t33208 * t34429 - 0.40208333333333333334e-2 * t112765 * t34429;
    (t118256,)
}
