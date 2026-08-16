//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 785/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk785(t1072: f64, t4547: f64, t10096: f64, t2844: f64, t822: f64, t102: f64, t4880: f64, t4859: f64, t23: f64, t821: f64, t6: f64, t107: f64) -> (f64, f64, f64, f64, f64) {
    let t13558 = 0.47822877300252710492e-1_f64 * t1072 * t4547;
    let t13564 = 0.62154466893555682512e-3_f64 * t10096 * t4547;
    let t13567 = t822 * t2844;
    let t13577 = t102 * t4880;
    let t13578 = t13577 * t4859;
    let t13581 = 1.0_f64 / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    (t13558, t13564, t13567, t13578, t13583)
}
