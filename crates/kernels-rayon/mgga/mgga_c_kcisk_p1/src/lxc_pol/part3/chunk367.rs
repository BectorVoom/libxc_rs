//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 367/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk367(t1636: f64, t1835: f64, t1379: f64, t435: f64, t690: f64, t579: f64, t79: f64) -> (f64, f64, f64) {
    let t1836 = t1835 * t1636;
    let t1841 = 0.7925e-3_f64 * t435 * t1379 * t690;
    let t1842 = t79 * t579;
    (t1836, t1841, t1842)
}
