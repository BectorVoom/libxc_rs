//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1012/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1012(t12501: f64, t639: f64, t7877: f64, t12869: f64, t1791: f64, t1006: f64, t10871: f64, t10969: f64, t2796: f64, t1022: f64, t1031: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t40824 = t639 * t7877 * t12501;
    let t40855 = t1791 * t12869;
    let t40865 = t1006 * t10871;
    let t40867 = t10969 * t2796;
    let t40899 = t1022 * t1031 * t184;
    (t40824, t40855, t40865, t40867, t40899)
}
