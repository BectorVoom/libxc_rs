//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 652/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk652(t5290: f64, t8946: f64, t5289: f64, t747: f64, t8672: f64, t746: f64, t5315: f64, t41: f64, t8831: f64, t719: f64, t734: f64, t2567: f64, t2571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9047 = t5290 * t8946;
    let t9048 = t5289 * t9047;
    let t9050 = t747 * t8672;
    let t9051 = t746 * t9050;
    let t9052 = t5315 * t9051;
    let t9054 = t8831 * t41;
    let t9055 = t9054 * t719;
    let t9056 = t734 * t9055;
    let t9058 = t2567 * t2571;
    (t9047, t9048, t9050, t9051, t9052, t9054, t9055, t9056, t9058)
}
