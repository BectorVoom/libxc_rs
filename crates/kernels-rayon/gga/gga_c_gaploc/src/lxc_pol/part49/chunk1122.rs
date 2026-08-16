//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1122/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1122(t313: f64, t314: f64, t317: f64, t47311: f64, t13876: f64, t2197: f64, t568: f64, t833: f64, t836: f64, t47187: f64, t701: f64, t1457: f64, t2004: f64) -> (f64, f64, f64, f64, f64) {
    let t47315 = 0.35750489951850426669e0_f64 * t313 * t314 * t47311 * t317;
    let t47317 = 0.23005755572352449806e1_f64 * t2197 * t13876;
    let t47321 = 0.23005755572352449806e1_f64 * t833 * t568 * t836 * t47311;
    let t47322 = t47187 * t701;
    let t47325 = 0.35750489951850426669e0_f64 * t2004 * t1457 * t47322;
    (t47315, t47317, t47321, t47322, t47325)
}
