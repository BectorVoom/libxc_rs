//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 661/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk661(t6670: f64, t824: f64, t2118: f64, t2263: f64, t358: f64, t356: f64, t2252: f64) -> (f64, f64, f64, f64) {
    let t6671 = t824 * t6670;
    let t6677 = t2118 * t6670;
    let t6683 = t358 * t2263;
    let t6684 = t356 * t6683;
    let t6685 = t6684 * t2252;
    (t6671, t6677, t6684, t6685)
}
