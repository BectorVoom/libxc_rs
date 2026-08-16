//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1170/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1170(t10673: f64, t11591: f64, t37505: f64, t10935: f64, t2810: f64, t3446: f64, t11563: f64, t2312: f64, t3447: f64, t158: f64, t2461: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t40428 = t10673 * t11591 * t37505;
    let t40434 = t3446 * t10935 * t2810;
    let t40451 = t3446 * t3447 * t11563 * t2312;
    let t40453 = t158 * t2461;
    let t40456 = t3446 * t3447 * t40453 * t874;
    (t40428, t40434, t40451, t40453, t40456)
}
