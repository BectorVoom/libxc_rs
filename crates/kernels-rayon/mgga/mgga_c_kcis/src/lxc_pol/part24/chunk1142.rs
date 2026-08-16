//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1142/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1142(t26470: f64, t91982: f64, t2398: f64, t68: f64, t26467: f64, t2725: f64, t26463: f64, t874: f64, t91978: f64, t91972: f64, t2157: f64, t37041: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91983 = t26470 * t91982;
    let t91985 = t2398 * t68;
    let t91987 = t2725 * t91985 * t26467;
    let t91989 = t26463 * t91982;
    let t91992 = t874 * t91985 * t26467;
    let t91994 = t26470 * t91978;
    let t91996 = t26470 * t91972;
    let t91999 = t874 * t37041 * t2157;
    (t91983, t91987, t91989, t91992, t91994, t91996, t91999)
}
