//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 984/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk984(t1017: f64, t11860: f64, t86: f64, t1392: f64, t9526: f64, t1398: f64, t1444: f64, t1494: f64, t2820: f64, t4158: f64) -> (f64, f64, f64, f64, f64) {
    let t11862 = t86 * t1017 * t11860;
    let t11881 = t86 * t9526 * t1392;
    let t11882 = t11881 * t1398;
    let t11898 = t1494 * t1444;
    let t11913 = t86 * t2820 * t4158;
    (t11862, t11881, t11882, t11898, t11913)
}
