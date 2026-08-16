//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 976/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk976(t1072: f64, t2630: f64, t2844: f64, t89: f64, t740: f64, t113: f64, t9494: f64, t1068: f64, t829: f64, t2635: f64, t331: f64, t1071: f64, t160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10091 = t1072 * t2630;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10097 = t10096 * t2630;
    let t10099 = t113 * t9494;
    let t10102 = t1068 * t829;
    let t10104 = t331 * t2635;
    let t10108 = t160 * t1071;
    (t10091, t10093, t10096, t10097, t10099, t10102, t10104, t10108)
}
