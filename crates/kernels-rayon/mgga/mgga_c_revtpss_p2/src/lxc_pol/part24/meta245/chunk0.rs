//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1007/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007(t1558: f64, t2811: f64, t2482: f64, t1531: f64, t37: f64, t1544: f64, t2475: f64, t124: f64, t136: f64, t243: f64, t220: f64, t10815: f64, t1561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    let t14613 = t37 * t1531;
    let t14648 = t2475 * t1544;
    let t14671 = t124 * t1558;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14712 = t10815 * t1561;
    (t14598, t14613, t14648, t14671, t14686, t14712)
}
