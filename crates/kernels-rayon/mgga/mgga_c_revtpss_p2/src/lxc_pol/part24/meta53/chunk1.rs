//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 357/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk357(t1333: f64, t512: f64, t520: f64, t72: f64, t757: f64, t177: f64) -> (f64, f64, f64, f64) {
    let t1334 = t512 * t1333;
    let t1337 = t520 * t72;
    let t1339 = 0.18311447306006545054e-3_f64 * t1337 * t757;
    let t1340 = t520 * t177;
    (t1334, t1337, t1339, t1340)
}
