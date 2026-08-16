//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 741/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk741(t2092: f64, t2096: f64, t4828: f64, t4831: f64, t4833: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4852: f64, t4854: f64, t4856: f64, t4858: f64, t4861: f64, t4864: f64) -> (f64, f64) {
    let t6080 = t2092 * t2096;
    let t6081 = 0.20538164420033333334e1_f64 * t6080;
    let t6082 = -t4828 + t4831 + t4833 - t4837 - t4840 - t4843 + t4846 + t4849 + t4852 - t4854 + t6081 + t4856 - t4858 + t4861 - t4864;
    (t6080, t6082)
}
