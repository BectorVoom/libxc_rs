//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 649/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk649(t8923: f64, t8955: f64, t8991: f64, t9028: f64, t82: f64, t72: f64, t739: f64, t9025: f64, t2031: f64, t4985: f64, t2320: f64, t7414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9030 = t8923 + t8955 + t8991 + t9028;
    let t9031 = t82 * t9030;
    let t9032 = t72 * t9031;
    let t9033 = t739 * t9025;
    let t9035 = t4985 * t2031;
    let t9037 = t7414 * t2320;
    (t9030, t9031, t9032, t9033, t9035, t9037)
}
