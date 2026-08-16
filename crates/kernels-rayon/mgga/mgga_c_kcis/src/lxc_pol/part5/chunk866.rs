//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 866/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk866(t1386: f64, t7091: f64, t1650: f64, t2001: f64, t4163: f64, t4162: f64, t4160: f64, t556: f64, t7053: f64, t553: f64, t303: f64, t1983: f64, t2006: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7092 = t7091 * t1386;
    let t7099 = t1650 * t2001;
    let t7100 = t4163 * t7099;
    let t7101 = t4162 * t7100;
    let t7102 = t4160 * t7101;
    let t7104 = t7053 * t556;
    let t7105 = t553 * t7104;
    let t7106 = t303 * t7105;
    let t7108 = t1983 * t2006;
    (t7092, t7101, t7102, t7104, t7105, t7106, t7108)
}
