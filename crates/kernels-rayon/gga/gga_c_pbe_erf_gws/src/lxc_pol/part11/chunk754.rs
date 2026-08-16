//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 754/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk754(t8018: f64, t8023: f64, t4826: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4854: f64, t4856: f64, t4858: f64, t4861: f64, t4864: f64) -> (f64, f64, f64) {
    let t12377 = 0.51947267698127589899e2_f64 * t8018;
    let t12378 = 0.35089340384731224426e1_f64 * t8023;
    let t12379 = -t12377 + t12378 + t4826 - t4837 - t4840 - t4843 + t4846 + t4849 + t4854 - t4856 - t4858 - t4861 - t4864;
    (t12377, t12378, t12379)
}
