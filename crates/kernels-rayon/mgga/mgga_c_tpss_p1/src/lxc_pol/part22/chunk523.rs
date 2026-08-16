//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 523/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk523(t2169: f64, t785: f64, t236: f64, t339: f64, t769: f64, t72: f64, t799: f64, t240: f64) -> (f64, f64, f64, f64) {
    let t2170 = t2169 * t785;
    let t2173 = t339 * t769 * t236;
    let t2174 = t799 * t72;
    let t2175 = t2174 * t240;
    (t2170, t2173, t2174, t2175)
}
