//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1579/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1579(t4677: f64, t4684: f64, t14506: f64, t3185: f64, t1932: f64, t3120: f64, t360: f64, t1629: f64, t1625: f64, t3040: f64, t3201: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14615 = t4677 * t4684;
    let t14618 = t14506 * t3185;
    let t14622 = t1932 * t3120 * t360;
    let t14623 = t1629 * t14622;
    let t14626 = t1625 * t3040;
    let t14627 = t14626 * t3201;
    let t14630 = t6739 * t3040 * t360;
    (t14615, t14618, t14622, t14623, t14626, t14627, t14630)
}
