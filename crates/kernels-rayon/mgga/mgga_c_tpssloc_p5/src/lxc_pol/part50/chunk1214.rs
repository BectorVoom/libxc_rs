//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1214/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1214(t3216: f64, t33013: f64, t1068: f64, t1070: f64, t113633: f64, t113637: f64, t113655: f64, t119016: f64, t119065: f64, t119107: f64, t119149: f64, t119440: f64, t119485: f64, t119529: f64, t119571: f64, t1637: f64, t193: f64, t23738: f64, t23742: f64, t25836: f64, t25845: f64, t30924: f64, t30930: f64, t336: f64, t4696: f64, t4700: f64, t6818: f64, t6822: f64, t7627: f64) -> f64 {
    let t119578 = t33013 * t3216;
    let t119608 = t193 * t336 * (t119016 + t119065 + t119107 + t119149 + t119440 + t119485 + t119529 + t119571) * t1070 - t4700 * t119578 * t1068 - t4700 * t113633 * t1637 + 2.0_f64 * t4700 * t113637 * t25845 - t4700 * t30924 * t4696 - 2.0_f64 * t4700 * t23738 * t7627 + 4.0_f64 * t4700 * t23742 * t7627 * t1068 - 2.0_f64 * t4700 * t6822 * t25836 + 4.0_f64 * t4700 * t23742 * t1637 * t6818 - 6.0_f64 * t4700 * t113655 * t25845 + 2.0_f64 * t4700 * t30930 * t4696;
    t119608
}
