//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2953/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953(t1063: f64, t1592: f64, t247: f64, t42778: f64, t11922: f64, t16044: f64, t3115: f64, t11714: f64, t11866: f64, t15716: f64, t15847: f64, t16078: f64, t16201: f64, t16205: f64, t16210: f64, t3106: f64, t3116: f64, t42477: f64, t42481: f64, t4808: f64, t53089: f64) -> f64 {
    let t53762 = t1063 * t247 * t42778 * t1592;
    let t53771 = t3115 * t11922 * t16044;
    let t53785 = 0.42344709252414555035e-4_f64 * t53762 - 0.22866142996303859718e-2_f64 * t3106 * t15847 - 0.38586616306262763275e-2_f64 * t15716 * t247 * t3116 * t53089 - 0.42874018118069736972e-3_f64 * t53771 - 0.64311027177104605458e-3_f64 * t11866 * t16078 + 0.22866142996303859718e-2_f64 * t42477 + 0.14291339372689912324e-3_f64 * t42481 + 0.22866142996303859718e-1_f64 * t3106 * t16201 - 0.76220476654346199061e-2_f64 * t11714 * t4808 - 0.3811023832717309953e-2_f64 * t3106 * t16205 - 0.10162730220579493208e-1_f64 * t3106 * t16210;
    t53785
}
