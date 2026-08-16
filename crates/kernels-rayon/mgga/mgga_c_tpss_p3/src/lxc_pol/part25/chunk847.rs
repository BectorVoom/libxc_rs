//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 847/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk847(t1338: f64, t1830: f64, t1812: f64, t6120: f64, t5826: f64, t5829: f64, t6124: f64, t6126: f64, t6128: f64) -> (f64, f64, f64) {
    let t6328 = t1830 * t1338;
    let t6331 = t1812 * t6120;
    let t6337 = -t5826 - t6124 / 24.0_f64 - t6126 / 768.0_f64 - t5829 - t6128 / 192.0_f64;
    (t6328, t6331, t6337)
}
