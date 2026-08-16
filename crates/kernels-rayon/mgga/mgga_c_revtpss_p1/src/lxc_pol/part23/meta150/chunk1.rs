//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 936/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk936(t4311: f64, t707: f64, t2498: f64, t2518: f64, t2522: f64, t2526: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t4300: f64, t4301: f64, t4304: f64, t4307: f64, t4310: f64) -> (f64, f64) {
    let t4313 = 4.0_f64 * t4311 * t707;
    let t4314 = t4300 - t2569 + t2579 + t2587 - t2522 - t2498 - t2518 - t4301 + t2526 + t2610 - t4304 - t2562 + t4307 + t4310 + t4313;
    (t4313, t4314)
}
