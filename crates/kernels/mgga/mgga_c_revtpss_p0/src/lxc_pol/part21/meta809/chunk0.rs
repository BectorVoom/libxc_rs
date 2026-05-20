//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2953/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953<F: Float>(t1063: F, t1592: F, t247: F, t42778: F, t11922: F, t16044: F, t3115: F, t11714: F, t11866: F, t15716: F, t15847: F, t16078: F, t16201: F, t16205: F, t16210: F, t3106: F, t3116: F, t42477: F, t42481: F, t4808: F, t53089: F) -> F {
    let t53762 = t1063 * t247 * t42778 * t1592;
    let t53771 = t3115 * t11922 * t16044;
    let t53785 = F::cast_from(0.42344709252414555035e-4_f64) * t53762 - F::cast_from(0.22866142996303859718e-2_f64) * t3106 * t15847 - F::cast_from(0.38586616306262763275e-2_f64) * t15716 * t247 * t3116 * t53089 - F::cast_from(0.42874018118069736972e-3_f64) * t53771 - F::cast_from(0.64311027177104605458e-3_f64) * t11866 * t16078 + F::cast_from(0.22866142996303859718e-2_f64) * t42477 + F::cast_from(0.14291339372689912324e-3_f64) * t42481 + F::cast_from(0.22866142996303859718e-1_f64) * t3106 * t16201 - F::cast_from(0.76220476654346199061e-2_f64) * t11714 * t4808 - F::cast_from(0.3811023832717309953e-2_f64) * t3106 * t16205 - F::cast_from(0.10162730220579493208e-1_f64) * t3106 * t16210;
    t53785
}
