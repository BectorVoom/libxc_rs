//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1272/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1272(t225: f64, t24698: f64, t480: f64, t1774: f64, t6622: f64, t1250: f64, t3720: f64, t6587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24699 = t24698 * t225;
    let t24700 = t24699 * t480;
    let t24704 = t1774 * t6622;
    let t24705 = t24704 * t1250;
    let t24706 = t3720 * t24705;
    let t24713 = t1774 * t6587;
    (t24699, t24700, t24704, t24705, t24706, t24713)
}
