//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1428/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1428(t2440: f64, t3966: f64, t4303: f64, t870: f64, t262: f64, t4119: f64, t157: f64, t9929: f64, t2430: f64, t4205: f64, t1409: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12877 = t2440 * t3966;
    let t12895 = t4303 * t870;
    let t12899 = t262 * t4119;
    let t12908 = t9929 * t157;
    let t12922 = 8.0_f64 * t4205 * t2430;
    let t12923 = t750 * t1409;
    (t12877, t12895, t12899, t12908, t12922, t12923)
}
