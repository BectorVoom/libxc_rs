//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3013/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3013(t11714: f64, t4817: f64, t12004: f64, t1042: f64, t1045: f64, t1063: f64, t11656: f64, t11774: f64, t15691: f64, t15847: f64, t16167: f64, t2858: f64, t3188: f64, t43204: f64, t43211: f64, t43215: f64, t43244: f64, t4788: f64, t4801: f64, t51958: f64, t53464: f64, t53474: f64, t999: f64) -> f64 {
    let t55070 = t11714 * t4817;
    let t55072 = t12004 * t4817;
    let t55096 = -0.30488190661738479624e-2_f64 * t55070 + 0.96545937095505185477e-2_f64 * t55072 + 0.19055119163586549765e-3_f64 * t43204 + 0.85748036236139473944e-3_f64 * t43211 - 0.85748036236139473944e-3_f64 * t1063 * t1042 * t4801 * t53464 - 0.34299214494455789578e-2_f64 * t1063 * t1042 * t51958 * t53474 + 0.22866142996303859718e-2_f64 * t11656 * t16167 + 0.42874018118069736972e-3_f64 * t43244 * t4788 + 0.42874018118069736972e-3_f64 * t3188 * t15847 + 0.10162730220579493208e-2_f64 * t43215 + 0.85748036236139473944e-3_f64 * t11774 * t15691 * t1045 * t2858 * t999;
    t55096
}
