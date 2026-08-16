//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1321/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1321(t1006: f64, t3683: f64, t823: f64, t1497: f64, t2116: f64, t20047: f64, t63884: f64, t18246: f64, t63859: f64, t44350: f64, t2428: f64, t10552: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t64914 = t823 * t1006 * t3683;
    let t64917 = t1497 * t2116;
    let t64923 = t20047 * t63884;
    let t64928 = t18246 * t63859;
    let t64941 = t20047 * t44350;
    let t64946 = t1497 * t2428;
    let t64950 = t33 * t10552;
    (t64914, t64917, t64923, t64928, t64941, t64946, t64950)
}
