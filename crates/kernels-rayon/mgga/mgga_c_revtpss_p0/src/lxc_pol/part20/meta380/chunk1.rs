//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1379/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379(t10638: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t10722: f64, t2656: f64, t2237: f64, t2482: f64, t849: f64, t2677: f64, t10489: f64, t221: f64, t2674: f64, t2675: f64) -> (f64, f64, f64, f64) {
    let t40705 = t2661 * t2662 * t243 * t10638 * t231;
    let t40707 = t10722 * t2656;
    let t40710 = t2482 * t849 * t2237;
    let t40711 = t40710 * t2677;
    let t40719 = t2674 * t2675 * t221 * t10489;
    (t40705, t40707, t40711, t40719)
}
