//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1196/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1196<F: Float>(t5005: F, t786: F, t1310: F, t2029: F, t4597: F, t6667: F, t10004: F, t9724: F, t2003: F, t2454: F, t20: F, t2801: F, t10005: F, t2807: F, t33008: F, t33196: F, t33270: F, t33278: F, t33279: F, t34201: F, t34204: F, t34469: F, t34548: F, t9725: F, t9728: F, t9740: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34559 = t5005 * t786;
    let t34560 = t1310 * t34559;
    let t34561 = t2029 * t4597;
    let t34562 = t34561 * t6667;
    let t34563 = t34560 * t34562;
    let t34573 = t9724 * t10004;
    let t34578 = t2003 * t2454;
    let t34579 = t34578 * t20;
    let t34580 = t2801 * t34579;
    let t34585 = -0.23148148148148148148e-2 * t9740 * t34563 + 0.6701388888888888889e-3 * t33196 * t34548 + 0.20104166666666666667e-2 * t9725 * t34469 + 0.17361111111111111111e-2 * t33270 - 0.13888888888888888889e-1 * t10005 * t9728 - 0.53611111111111111112e-2 * t34573 * t9728 + 0.11607361111111111111e-2 * t33008 + t33278 - 0.17361111111111111111e-2 * t33279 + 0.13888888888888888889e-1 * t34580 * t2807 - 0.11607361111111111111e-2 * t34201 + 0.77382407407407407407e-3 * t34204;
    (t34559, t34560, t34561, t34562, t34563, t34573, t34578, t34579, t34580, t34585)
}
