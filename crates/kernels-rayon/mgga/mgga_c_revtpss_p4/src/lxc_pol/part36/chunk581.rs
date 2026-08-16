//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 581/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk581(t4303: f64, t4306: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2628: f64, t2632: f64) -> (f64, f64, f64) {
    let t5924 = 0.36622894612013090108e-3_f64 * t4303;
    let t5925 = 8.0_f64 * t4306;
    let t5926 = -t2569 + t2579 + t2587 - t2522 - t2498 - t2518 + t2610 - t5924 - t2562 + t5925 + t2632 + t2628;
    (t5924, t5925, t5926)
}
