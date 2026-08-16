//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1027/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1027(t112620: f64, t112621: f64, t112622: f64, t115681: f64, t115684: f64, t115690: f64, t115695: f64, t115698: f64, t115700: f64, t115702: f64, t2040: f64, t23951: f64, t24028: f64, t24169: f64, t24545: f64, t24932: f64, t27888: f64, t7042: f64, t7050: f64, t7057: f64, t8690: f64, t94248: f64, t96222: f64) -> f64 {
    let t117622 = -2.0_f64 * t2040 * t94248 - 4.0_f64 * t2040 * t96222 - t23951 * t8690 - 2.0_f64 * t24028 * t8690 + 2.0_f64 * t24169 * t8690 - 4.0_f64 * t24545 * t7042 - 4.0_f64 * t24932 * t7057 - 4.0_f64 * t27888 * t7050 - t112620 - t112621 - t112622 + t115681 + t115684 + t115690 + t115695 - t115698 + t115700 - t115702;
    t117622
}
