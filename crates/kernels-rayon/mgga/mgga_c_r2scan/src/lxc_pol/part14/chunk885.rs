//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 885/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk885(t6086: f64, t8081: f64, t6085: f64, t7619: f64, t6093: f64, t1567: f64, t2115: f64, t494: f64, t7338: f64) -> (f64, f64, f64, f64) {
    let t8082 = t6086 * t8081;
    let t8084 = 0.11643651550782197811e-1_f64 * t6085 * t8082;
    let t8085 = t6086 * t7619;
    let t8086 = t6093 * t8085;
    let t8088 = t2115 * t1567;
    let t8089 = t7338 * t494;
    (t8084, t8086, t8088, t8089)
}
