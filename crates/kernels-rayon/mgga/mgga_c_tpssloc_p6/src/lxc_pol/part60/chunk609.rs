//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 609/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk609(t5: f64, t2031: f64, t7445: f64, t1860: f64, t2032: f64, t7026: f64, t7034: f64, t7428: f64, t7432: f64, t7435: f64, t112: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7782 = t2031 * t7445;
    let t7786 = piecewise3(t8, 0.0_f64, t7428 * t2032 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t7026 * t7432 - 2.0_f64 / 3.0_f64 * t7435 * t2032 - t7034 + t1860 * t7782 / 3.0_f64);
    let t7787 = t7786 * t112;
    (t7782, t7786, t7787)
}
