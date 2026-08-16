//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2912/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912(t324: f64, t77549: f64, t77596: f64, t300: f64, t1633: f64, t52894: f64, t64043: f64, t972: f64, t19331: f64, t52514: f64, t1610: f64, t63610: f64) -> (f64, f64, f64, f64, f64) {
    let t77598 = (t77549 + t77596) * t324;
    let t77600 = 0.19751673498613801407e-1_f64 * t300 * t77598;
    let t77604 = 0.30762056574649219973e4_f64 * t52894 * t64043 * t1633 * t972;
    let t77612 = 0.2894756309764656312e3_f64 * t52514 * t19331;
    let t77622 = 3.0_f64 * t63610 * t1610;
    (t77598, t77600, t77604, t77612, t77622)
}
