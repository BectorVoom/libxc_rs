//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 950/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk950(t2441: f64, t2877: f64, t8072: f64, t895: f64, t3371: f64, t528: f64, t1564: f64, t3338: f64, t475: f64, t1445: f64, t10152: f64, t1457: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10334 = 0.35750489951850426669e0_f64 * t2441 * t2877;
    let t10336 = 0.35750489951850426669e0_f64 * t895 * t8072;
    let t10337 = t528 * t3371;
    let t10340 = t1564 * t3338;
    let t10341 = t10340 * t475;
    let t10342 = t1445 * t10341;
    let t10345 = t1457 * t10152;
    (t10334, t10336, t10337, t10340, t10341, t10342, t10345)
}
