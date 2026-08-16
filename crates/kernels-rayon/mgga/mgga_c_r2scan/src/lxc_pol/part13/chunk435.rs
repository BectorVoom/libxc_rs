//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 435/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk435(t124: f64, t182: f64, t190: f64, t625: f64, t406: f64, t726: f64, t58: f64, t583: f64) -> (f64, f64, f64, f64) {
    let t1853 = t124 * t182;
    let t1856 = 0.23744444444444444444e-1_f64 * t625 * t1853 * t190;
    let t1858 = 8.0_f64 * t406 * t726;
    let t1859 = t583 * t58;
    (t1853, t1856, t1858, t1859)
}
