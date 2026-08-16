//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1339/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1339(t32436: f64, t8709: f64, t8710: f64, t8711: f64, t8713: f64, t8715: f64, t9128: f64, t9129: f64, t9744: f64, t9746: f64, t9748: f64) -> f64 {
    let tv4rho43 = 3.0_f64 * t8709 + 3.0_f64 * t8710 + 6.0_f64 * t8711 + 6.0_f64 * t8713 + 3.0_f64 * t8715 + 3.0_f64 * t9128 + 0.1434375e0_f64 * t9129 - 0.7171875e-1_f64 * t9744 - 0.4303125e0_f64 * t9746 + 0.286875e0_f64 * t9748 + t32436;
    tv4rho43
}
