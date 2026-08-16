//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1105/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1105(t1142: f64, t29081: f64, t2205: f64, t6879: f64, t1872: f64, t8117: f64, t20711: f64, t29025: f64, t29027: f64, t29030: f64, t29033: f64, t29035: f64, t29038: f64, t29044: f64, t3669: f64, t5360: f64) -> (f64, f64, f64, f64) {
    let t29082 = t1142 * t29081;
    let t29084 = t2205 * t6879;
    let t29087 = t8117 * t1872;
    let t29092 = -t20711 * t2205 + 2.0_f64 * t29084 * t3669 + 4.0_f64 * t29087 * t3669 - 2.0_f64 * t5360 * t8117 - t29025 + t29027 + t29030 - t29033 + t29035 + t29038 - t29044;
    (t29082, t29084, t29087, t29092)
}
