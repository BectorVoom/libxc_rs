//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1010/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1010(t6080: f64, t4826: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4856: f64, t4864: f64, t8022: f64, t8024: f64, t8026: f64, t8027: f64, t8028: f64, t8031: f64, t8032: f64, t8033: f64, t8034: f64, t8035: f64) -> f64 {
    let t9047 = 0.13692109613355555556e1_f64 * t6080;
    let t9048 = -t8022 + t8024 - t8026 - t8027 + t4826 + t8028 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t8032 - t8033 + t9047 - t4856 + t8034 + t8035 - t4864;
    t9048
}
