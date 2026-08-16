//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1445/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1445(t122660: f64, t2040: f64, t33363: f64, t7000: f64, t115774: f64, t1983: f64, t7687: f64, t1307: f64, t22574: f64, t26558: f64, t33221: f64, t12461: f64, t8639: f64) -> (f64, f64, f64, f64, f64) {
    let t122662 = 2.0_f64 * t122660 * t2040;
    let t122664 = t33363 * t7000;
    let t122667 = 3.0_f64 * t1983 * t115774 * t7687;
    let t122671 = 6.0_f64 * t22574 * t26558 * t33221 * t1307;
    let t122675 = t8639 * t12461;
    (t122662, t122664, t122667, t122671, t122675)
}
