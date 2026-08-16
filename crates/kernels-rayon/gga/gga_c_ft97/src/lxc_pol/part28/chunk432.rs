//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 432/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk432(t140: f64, t5579: f64, t6608: f64, t1355: f64, t2043: f64, t5785: f64, t5802: f64, t5813: f64, t5829: f64, t5837: f64, t5838: f64, t6450: f64, t6593: f64, t6597: f64, t6605: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t6609 = t5579 * t6608;
    let t6615 = piecewise3(t141, 0.45306850413028723348e0_f64 * t5785 * t6593 - 0.22653425206514361674e0_f64 * t2043 * t6597 - 0.45306850413028723348e0_f64 * t5802 * t6593 + 0.22653425206514361674e0_f64 * t1355 * t6597 - 0.10001700163888888889e0_f64 * t5813 * t6605 + 0.10001700163888888889e0_f64 * t5829 * t6609 - t5837 - 0.16669500273148148149e-1_f64 * t5838 * t6450, 0.0_f64);
    t6615
}
