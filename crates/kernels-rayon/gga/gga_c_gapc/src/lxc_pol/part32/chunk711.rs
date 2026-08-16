//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 711/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk711(t4533: f64, t8379: f64, t1947: f64, t2937: f64, t1928: f64, t1936: f64, t2941: f64, t1587: f64, t2880: f64, t2879: f64, t2885: f64, t507: f64) -> (f64, f64, f64, f64, f64) {
    let t8380 = t8379 * t4533;
    let t8381 = t2937 * t1947;
    let t8382 = t8380 * t8381;
    let t8384 = t1936 * t1928;
    let t8385 = t2941 * t8384;
    let t8387 = t2880 * t1587;
    let t8388 = t2879 * t8387;
    let t8390 = t2885 * t507;
    (t8381, t8382, t8385, t8388, t8390)
}
