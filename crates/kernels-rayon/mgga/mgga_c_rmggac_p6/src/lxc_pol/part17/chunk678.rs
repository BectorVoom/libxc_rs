//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 678/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk678(t675: f64, t9795: f64, t1743: f64, t649: f64, t27: f64, t2139: f64, t1756: f64, t2145: f64, t1734: f64, t2134: f64, t2333: f64, t8368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9796 = t675 * t9795;
    let t9797 = 0.12769379967989351819e-4_f64 * t9796;
    let t9798 = t649 * t1743;
    let t9799 = t27 * t9798;
    let t9800 = t2139 * t9799;
    let t9801 = 0.13637330827122670864e-1_f64 * t9800;
    let t9802 = t649 * t1756;
    let t9803 = t27 * t9802;
    let t9804 = t2145 * t9803;
    let t9805 = 0.34093327067806677161e-2_f64 * t9804;
    let t9806 = t649 * t1734;
    let t9807 = t27 * t9806;
    let t9808 = t2134 * t9807;
    let t9809 = 0.10227998120342003148e-1_f64 * t9808;
    let t9810 = t8368 * t2333;
    (t9797, t9799, t9801, t9803, t9805, t9807, t9809, t9810)
}
