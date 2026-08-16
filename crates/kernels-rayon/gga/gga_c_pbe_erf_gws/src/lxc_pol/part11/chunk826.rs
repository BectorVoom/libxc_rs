//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 826/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk826(t12374: f64, t12375: f64, t12377: f64, t12378: f64, t4826: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4854: f64, t4856: f64, t4858: f64, t4861: f64, t4864: f64) -> f64 {
    let t13154 = -t12374 - t12375 - t12377 + t12378 + t4826 - t4837 - t4840 - t4843 + t4846 + t4849 + t4854 - t4856 - t4858 - t4861 - t4864;
    t13154
}
