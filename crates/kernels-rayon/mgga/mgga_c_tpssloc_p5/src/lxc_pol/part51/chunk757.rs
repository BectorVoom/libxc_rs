//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 757/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk757(t5: f64, t1860: f64, t7032: f64, t2031: f64, t6509: f64, t2032: f64, t6486: f64, t6492: f64, t6495: f64, t7026: f64, t112: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7034 = 8.0_f64 / 9.0_f64 * t1860 * t7032;
    let t7035 = t2031 * t6509;
    let t7039 = piecewise3(t8, 0.0_f64, t6486 * t2032 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t7026 * t6492 - 2.0_f64 / 3.0_f64 * t6495 * t2032 - t7034 + t1860 * t7035 / 3.0_f64);
    let t7040 = t7039 * t112;
    (t7034, t7035, t7039, t7040)
}
