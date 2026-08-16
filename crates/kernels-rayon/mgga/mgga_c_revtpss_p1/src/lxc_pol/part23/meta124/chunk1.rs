//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 811/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk811(t281: f64, t2902: f64, t414: f64, t1146: f64, t698: f64, t1224: f64, t240: f64) -> (f64, f64, f64, f64) {
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0_f64 * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    (t3413, t3414, t3415, t3417)
}
