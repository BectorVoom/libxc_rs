//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 978/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk978(t339: f64, t5685: f64, t8495: f64, t8497: f64, t1064: f64, t1775: f64, t8510: f64, t8518: f64, t8524: f64, t1067: f64, t1765: f64, t2737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14423 = 24.0_f64 * t339 * t5685;
    let t14432 = 36.0_f64 * t8495;
    let t14433 = 96.0_f64 * t8497;
    let t14435 = t1064 * t1775;
    let t14437 = 96.0_f64 * t8510;
    let t14439 = 960.0_f64 * t8518;
    let t14440 = 192.0_f64 * t8524;
    let t14443 = t1067 * t1775;
    let t14444 = 36.0_f64 * t14443;
    let t14445 = t1765 * t2737;
    (t14423, t14432, t14433, t14435, t14437, t14439, t14440, t14444, t14445)
}
