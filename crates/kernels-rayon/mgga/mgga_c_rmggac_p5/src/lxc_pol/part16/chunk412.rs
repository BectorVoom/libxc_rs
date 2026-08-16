//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 412/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk412(t31: f64, t3899: f64, t27: f64, t32: f64, t124: f64, t128: f64, t325: f64, t899: f64) -> (f64, f64, f64, f64, f64) {
    let t3900 = t31 * t3899;
    let t3901 = 308.0_f64 / 27.0_f64 * t3900;
    let t3907 = t27 * t32 * t3899;
    let t3908 = 0.57037037037037037036e0_f64 * t3907;
    let t3924 = t124 * t128;
    let t3928 = t899 * t325;
    (t3900, t3901, t3908, t3924, t3928)
}
