//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1235/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1235(t1386: f64, t3663: f64, t3665: f64, t2981: f64, t34754: f64, t458: f64, t11425: f64, t568: f64, t2974: f64, t34925: f64, t8675: f64, t1625: f64, t5296: f64) -> (f64, f64, f64, f64, f64) {
    let t35240 = t1386 * t3663 * t3665;
    let t35243 = t34754 * t2981 * t458;
    let t35246 = t11425 * t2981 * t568;
    let t35249 = t34925 * t8675 * t2974;
    let t35251 = t5296 * t1625;
    (t35240, t35243, t35246, t35249, t35251)
}
