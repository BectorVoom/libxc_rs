//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 370/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk370(t1635: f64, t2366: f64, t587: f64, t1634: f64) -> (f64, f64, f64) {
    let t2368 = -t1635 - 0.17808333333333333333e-1_f64 * t2366;
    let t2370 = 0.62182e-1_f64 * t2368 * t587;
    let t2372 = -t1634 / 3.0_f64 - t2366 / 3.0_f64;
    (t2368, t2370, t2372)
}
