//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 994/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk994(t1882: f64, t33127: f64, t33167: f64, t33211: f64, t33087: f64, t33072: f64, t33176: f64, t7363: f64, t8232: f64, t582: f64, t7390: f64, t33046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140078 = t1882 * t33127;
    let t140087 = t1882 * t33167;
    let t140089 = t1882 * t33211;
    let t140094 = t1882 * t33087;
    let t140103 = t1882 * t33072;
    let t140112 = t1882 * t33176;
    let t140129 = 4.0_f64 / 27.0_f64 * t8232 * t7363;
    let t140137 = t582 * t7390;
    let t140144 = t1882 * t33046;
    (t140078, t140087, t140089, t140094, t140103, t140112, t140129, t140137, t140144)
}
