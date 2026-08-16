//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1141/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1141(t41668: f64, t12797: f64, t2615: f64, t2559: f64, t47446: f64, t587: f64, t34544: f64, t48148: f64, t48150: f64, t48152: f64, t48153: f64, t48155: f64, t48158: f64, t48159: f64) -> (f64, f64, f64, f64) {
    let t48160 = 64.0_f64 / 45.0_f64 * t41668;
    let t48162 = 16.0_f64 / 9.0_f64 * t2615 * t12797;
    let t48165 = 16.0_f64 / 27.0_f64 * t587 * t2559 * t47446;
    let t48166 = -t48148 + t48150 + 0.72933333333333333331e0_f64 * t34544 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162 + t48165;
    (t48160, t48162, t48165, t48166)
}
