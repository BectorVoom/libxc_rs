//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 902/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk902(t10151: f64, t1063: f64, t2343: f64, t6519: f64, t2268: f64, t8195: f64, t9189: f64, t2854: f64, t29975: f64, t6320: f64, t24139: f64, t8124: f64) -> (f64, f64, f64, f64) {
    let t42625 = t1063 * t2343 * t10151 * t6519;
    let t42629 = 0.19918504644973304719e0_f64 * t2268 * t9189 * t8195;
    let t42633 = 0.17073003981405689759e1_f64 * t2268 * t6320 * t2854 * t29975;
    let t42637 = 0.68292015925622759036e0_f64 * t2268 * t24139 * t8124 * t29975;
    (t42625, t42629, t42633, t42637)
}
