//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1330/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1330(t16552: f64, t21130: f64, t21134: f64, t5425: f64, t531: f64, t7141: f64, t833: f64, t3766: f64, t6964: f64, t3761: f64, t7122: f64, t11322: f64) -> (f64, f64, f64, f64, f64) {
    let t21993 = t16552 * t21130;
    let t21996 = t5425 * t21134;
    let t21999 = t7141 * t531;
    let t22000 = t21999 * t833;
    let t22001 = t3766 * t22000;
    let t22004 = t6964 * t531;
    let t22005 = t22004 * t833;
    let t22006 = t3761 * t22005;
    let t22009 = t7122 * t531;
    let t22010 = t22009 * t833;
    let t22011 = t11322 * t22010;
    (t21993, t21996, t22001, t22006, t22011)
}
