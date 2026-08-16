//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2932/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932(t17579: f64, t225: f64, t18048: f64, t210: f64, t974: f64, t2985: f64, t1597: f64, t976: f64, t17826: f64, t2960: f64, t12652: f64, t4337: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61058 = t17579 * t225;
    let t61061 = t18048 * t225;
    let t61064 = t210 * t974;
    let t61065 = t2985 * t61064;
    let t61066 = t976 * t1597;
    let t61074 = t2960 * t17826;
    let t61078 = t4337 * t12652;
    (t61058, t61061, t61064, t61065, t61066, t61074, t61078)
}
