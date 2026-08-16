//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1391/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1391(t6798: f64, t8411: f64, t10351: f64, t20496: f64, t20671: f64, t27003: f64, t31041: f64, t10597: f64, t31051: f64, t2482: f64, t8272: f64, t9267: f64) -> (f64, f64, f64, f64, f64) {
    let t34621 = 0.14300195980740170668e1_f64 * t8411 * t6798;
    let t34623 = 0.13803453343411469884e2_f64 * t20496 * t10351;
    let t34625 = t31041 * t20671 * t27003;
    let t34626 = 0.17041300423964777634e0_f64 * t34625;
    let t34627 = t31051 * t10597;
    let t34628 = 0.19171462976960374838e1_f64 * t34627;
    let t34630 = t9267 * t8272 * t2482;
    (t34621, t34623, t34626, t34628, t34630)
}
