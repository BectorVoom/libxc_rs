//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 980/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk980(t7622: f64, t8232: f64, t1882: f64, t34133: f64, t34104: f64, t7669: f64, t34164: f64, t34111: f64, t34146: f64, t34150: f64, t34099: f64, t2842: f64, t7679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t144060 = 8.0_f64 / 27.0_f64 * t8232 * t7622;
    let t144073 = t1882 * t34133;
    let t144087 = t1882 * t34104;
    let t144093 = 8.0_f64 / 27.0_f64 * t8232 * t7669;
    let t144094 = t1882 * t34164;
    let t144096 = t1882 * t34111;
    let t144105 = t1882 * t34146;
    let t144107 = t1882 * t34150;
    let t144123 = t1882 * t34099;
    let t144131 = t2842 * t7679;
    (t144060, t144073, t144087, t144093, t144094, t144096, t144105, t144107, t144123, t144131)
}
