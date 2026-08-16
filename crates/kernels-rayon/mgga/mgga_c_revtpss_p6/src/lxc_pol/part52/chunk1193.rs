//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1193/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1193(t1711: f64, t7086: f64, t125961: f64, t27799: f64, t27363: f64, t33: f64, t126017: f64, t196: f64, t197: f64, t28230: f64, t13426: f64, t8461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    let t127284 = t27799 * t126017;
    let t127317 = t28230 * t196 * t197;
    let t127365 = t13426 * t8461;
    (t127212, t127218, t127227, t127284, t127317, t127365)
}
