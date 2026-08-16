//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1197/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1197(t1711: f64, t7086: f64, t125961: f64, t27799: f64, t27363: f64, t33: f64, t126017: f64, t13426: f64, t8461: f64, t18227: f64, t32110: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    let t127284 = t27799 * t126017;
    let t127365 = t13426 * t8461;
    let t127366 = 2.0_f64 * t127365;
    let t127368 = t18227 * t8461;
    let t127369 = 2.0_f64 * t127368;
    let t127370 = t4248 * t32110;
    (t127212, t127218, t127227, t127284, t127366, t127369, t127370)
}
