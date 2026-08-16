//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1125/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1125(t3668: f64, t6856: f64, t3217: f64, t6555: f64, t1851: f64, t5336: f64, t1262: f64, t6774: f64, t6837: f64, t6496: f64, t9545: f64, t19904: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67159 = t6856 * t3668;
    let t67493 = t3217 * t6555;
    let t68040 = t1851 * t5336;
    let t68045 = t6774 * t1262;
    let t68901 = t6837 * t1262;
    let t69078 = t9545 * t6496;
    let t69377 = t19904 * sigma0;
    (t67159, t67493, t68040, t68045, t68901, t69078, t69377)
}
