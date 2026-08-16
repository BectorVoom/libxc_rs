//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 962/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk962(t2877: f64, t984: f64, t2810: f64, t296: f64, t1080: f64, t2475: f64, t2844: f64, t89: f64, t740: f64, t113: f64, t9494: f64, t1068: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9970 = t984 * t2877;
    let t9985 = 1.0_f64 / t2810 / t296;
    let t10033 = t2475 * t1080;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10099 = t113 * t9494;
    let t10102 = t1068 * t829;
    (t9970, t9985, t10033, t10093, t10096, t10099, t10102)
}
