//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2545/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545(t1102: f64, t5999: f64, t14801: f64, t14804: f64, t45192: f64, t48140: f64, t68513: f64, t50822: f64, t44938: f64, t43777: f64, t43859: f64, t43895: f64, t50919: f64, t50948: f64, t71203: f64, t71206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71498 = t5999 * t1102;
    let t71499 = t14801 * t71498;
    let t71501 = t14804 * t71498;
    let t71505 = t48140 * t45192 * t68513;
    let t71508 = t48140 * t50822 * t68513;
    let t71511 = t48140 * t44938 * t68513;
    let t71515 = 0.181155e1_f64 * t71203 + 0.543465e1_f64 * t71206 + t43777 + 0.58258125e1_f64 * t71499 - 0.1237865625e0_f64 * t71501 - 0.24528888888888888889e0_f64 * t43859 - 0.49671e0_f64 * t71505 + 0.149013e1_f64 * t71508 + 0.11038e0_f64 * t71511 - 0.26837777777777777779e0_f64 * t50919 + 0.80513333333333333336e0_f64 * t50948 + t43895;
    (t71499, t71501, t71505, t71508, t71511, t71515)
}
