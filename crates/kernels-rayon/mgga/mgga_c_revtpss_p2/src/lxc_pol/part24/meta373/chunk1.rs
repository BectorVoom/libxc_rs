//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1265/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265(t1715: f64, t21093: f64, t1042: f64, t1774: f64, t5819: f64, t5268: f64, t6573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24604 = t21093 * t1715;
    let t24605 = t1042 * t24604;
    let t24610 = t5819 * t1774;
    let t24611 = t5268 * t24610;
    let t24612 = t1042 * t24611;
    let t24616 = t6573 * t1774;
    (t24604, t24605, t24610, t24611, t24612, t24616)
}
