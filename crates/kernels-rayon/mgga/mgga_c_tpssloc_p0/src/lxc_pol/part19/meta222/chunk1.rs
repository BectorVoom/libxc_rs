//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 924/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk924(t10620: f64, t10649: f64, t10652: f64, t10654: f64, t10657: f64, t10665: f64, t10699: f64, t10707: f64, t10771: f64, t10772: f64, t10806: f64, t10811: f64, t10814: f64, t10819: f64, t10820: f64, t10825: f64, t10828: f64, t10829: f64, t10843: f64, t2900: f64, t2925: f64, t2933: f64, t311: f64, t924: f64, t952: f64) -> f64 {
    let t10847 = -0.19298375398431042081e3_f64 * t10771 * t10772 + 1.0_f64 * t924 * t10806 + 0.2069040516770936012e4_f64 * t10811 * t10814 + t10819 + t10649 - t10652 - t10654 - t10657 + t10665 - t10699 - t10707 + 0.17544670867903938621e1_f64 * t10820 * t952 + 0.17544670867903938621e1_f64 * t2900 * t2925 + 0.51947577317044391276e2_f64 * t10825 * t2933 - 0.10389515463408878255e3_f64 * t10828 * t10829 - 0.310907e-1_f64 * t10843 * t311 - 0.19751673498613801407e-1_f64 * t10620;
    t10847
}
