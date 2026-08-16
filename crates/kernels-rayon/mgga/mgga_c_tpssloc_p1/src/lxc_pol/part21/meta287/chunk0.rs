//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1587/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1587(t10422: f64, t3072: f64, t3070: f64, t1005: f64, t3082: f64, t1036: f64, t3094: f64, t3089: f64, t248: f64, t2780: f64, t3051: f64, t1041: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10423 = t10422 * t3072;
    let t10424 = t3070 * t10423;
    let t10436 = t1005 * t3082;
    let t10441 = t3094 * t1036;
    let t10449 = t3089 * t1036;
    let t10454 = t248 * t3051 * t2780;
    let t10455 = t1041 * t10454;
    (t10423, t10424, t10436, t10441, t10449, t10454, t10455)
}
