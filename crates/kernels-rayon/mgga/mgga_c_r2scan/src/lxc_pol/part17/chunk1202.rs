//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1202/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1202(t10935: f64, t3165: f64, t3446: f64, t158: f64, t3128: f64, t3447: f64, t874: f64, t122: f64, t3434: f64, t3437: f64, t797: f64, t8629: f64) -> (f64, f64, f64, f64) {
    let t43921 = t3446 * t10935 * t3165;
    let t43936 = t158 * t3128;
    let t43939 = t3446 * t3447 * t43936 * t874;
    let t43943 = t3434 * t3437 * t43936 * t122;
    let t43950 = t797 * t8629;
    (t43921, t43939, t43943, t43950)
}
