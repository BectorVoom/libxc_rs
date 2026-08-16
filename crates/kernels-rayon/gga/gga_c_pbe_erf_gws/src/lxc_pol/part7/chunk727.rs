//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 727/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk727(t5899: f64, t5914: f64, t5916: f64, t5923: f64, t5930: f64, t5956: f64, t5958: f64, t5959: f64, t163: f64, t169: f64, t684: f64, t784: f64) -> (f64, f64) {
    let t5962 = t5899 + t5914 + t5916 + t5923 + t5930 + t5956 + t5958 + t5959;
    let t5969 = t169 * t784 * t684 * t163;
    (t5962, t5969)
}
