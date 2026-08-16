//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1029/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1029(t33363: f64, t7756: f64, t33623: f64, t7685: f64, t101138: f64, t26161: f64, t33221: f64, t1983: f64, t20085: f64, t8640: f64, t29377: f64, t8643: f64) -> (f64, f64, f64, f64, f64) {
    let t128577 = 2.0_f64 * t33363 * t7756;
    let t128581 = 2.0_f64 * t7685 * t33623;
    let t128584 = 4.0_f64 * t26161 * t101138 * t33221;
    let t128588 = 2.0_f64 * t1983 * t8640 * t20085;
    let t128592 = t1983 * t29377 * t8643;
    (t128577, t128581, t128584, t128588, t128592)
}
