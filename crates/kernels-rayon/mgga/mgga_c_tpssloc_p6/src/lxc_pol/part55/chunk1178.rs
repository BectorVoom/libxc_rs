//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1178/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1178(t234: f64, t240: f64, t241: f64, t4248: f64, t776: f64, t812: f64, t9646: f64, t4234: f64, t6605: f64, t6612: f64, t25119: f64, t4255: f64, t6619: f64) -> (f64, f64, f64) {
    let t118546 = t812 * t234 * t240 * t241 * t9646 * t4248 * t776;
    let t118549 = t6605 * t6612 * t4234;
    let t118552 = t25119 * t6619 * t4255;
    (t118546, t118549, t118552)
}
