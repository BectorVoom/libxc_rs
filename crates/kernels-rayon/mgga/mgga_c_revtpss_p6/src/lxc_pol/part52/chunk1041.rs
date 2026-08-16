//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1041/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1041(t31832: f64, t31849: f64, t31798: f64, t8477: f64) -> (f64, f64, f64) {
    let t32458 = 0.66119071333692697238e-4_f64 * t31832;
    let t32460 = 0.17354086964223805049e-2_f64 * t31849;
    let t32463 = t8477 * t31798;
    (t32458, t32460, t32463)
}
