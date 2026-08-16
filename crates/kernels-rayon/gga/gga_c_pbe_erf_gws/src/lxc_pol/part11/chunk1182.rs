//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1182/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1182(t32097: f64, t41184: f64, t47825: f64, t47828: f64, t47832: f64, t47836: f64, t47839: f64, t47841: f64, t47844: f64, t47848: f64, t47850: f64, t47851: f64) -> f64 {
    let t48651 = -t47825 - t47828 - t47832 - t47836 + t47839 - t47841 - t47844 - t47848 - t47850 - t47851 + 0.43284165449459373508e0_f64 * t32097 + 16.0_f64 / 3.0_f64 * t41184;
    t48651
}
