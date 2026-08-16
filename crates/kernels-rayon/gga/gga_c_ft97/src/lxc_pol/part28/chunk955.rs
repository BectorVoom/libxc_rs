//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 955/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk955(t23054: f64, t32117: f64, t2: f64, t32325: f64, t1317: f64, t32121: f64, t376: f64, t32087: f64, t5665: f64, t1882: f64, t32335: f64, t32063: f64, t32083: f64, t7238: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t137110 = t23054 * t32117;
    let t137112 = t2 * t32325;
    let t137124 = t1317 * t376 * t32121;
    let t137131 = t5665 * t376 * t32087;
    let t137163 = t1882 * t32335;
    let t137172 = t7238 * t32063 * t32083;
    (t137110, t137112, t137124, t137131, t137163, t137172)
}
