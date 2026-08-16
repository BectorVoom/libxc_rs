//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 417/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk417(t1415: f64, t847: f64, t854: f64, t1407: f64, t861: f64, t141: f64, t1409: f64, t852: f64, t860: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1416 = t847 * t1415;
    let t1419 = t854 * t1415;
    let t1421 = t861 * t1407;
    let t1422 = t141 * t1421;
    let t1424 = 0.1898925e1_f64 * t1416 - t852 - 0.29896666666666666667e0_f64 * t1409 + 0.3071625e0_f64 * t1419 - t860 - 0.82156666666666666667e-1_f64 * t1422;
    let t1425 = t1424 * t866;
    (t1416, t1419, t1421, t1422, t1424, t1425)
}
