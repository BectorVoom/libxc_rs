//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1077/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1077(t14600: f64, t676: f64, t836: f64, t14598: f64, t1558: f64, t879: f64, t2482: f64, t2801: f64, t1531: f64, t37: f64, t4392: f64, t72: f64) -> (f64, f64, f64, f64) {
    let t14602 = t14600 * t676 * t836;
    let t14603 = t14598 * t14602;
    let t14605 = t879 * t1558;
    let t14606 = t2482 * t14605;
    let t14608 = 0.19514881078765566038e-1_f64 * t14606 * t2801;
    let t14613 = t37 * t1531;
    let t14616 = t4392 * t72;
    (t14603, t14608, t14613, t14616)
}
