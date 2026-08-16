//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3013/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3013<F: Float>(t11714: F, t4817: F, t12004: F, t1042: F, t1045: F, t1063: F, t11656: F, t11774: F, t15691: F, t15847: F, t16167: F, t2858: F, t3188: F, t43204: F, t43211: F, t43215: F, t43244: F, t4788: F, t4801: F, t51958: F, t53464: F, t53474: F, t999: F) -> F {
    let t55070 = t11714 * t4817;
    let t55072 = t12004 * t4817;
    let t55096 = -F::cast_from(0.30488190661738479624e-2_f64) * t55070 + F::cast_from(0.96545937095505185477e-2_f64) * t55072 + F::cast_from(0.19055119163586549765e-3_f64) * t43204 + F::cast_from(0.85748036236139473944e-3_f64) * t43211 - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t4801 * t53464 - F::cast_from(0.34299214494455789578e-2_f64) * t1063 * t1042 * t51958 * t53474 + F::cast_from(0.22866142996303859718e-2_f64) * t11656 * t16167 + F::cast_from(0.42874018118069736972e-3_f64) * t43244 * t4788 + F::cast_from(0.42874018118069736972e-3_f64) * t3188 * t15847 + F::cast_from(0.10162730220579493208e-2_f64) * t43215 + F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15691 * t1045 * t2858 * t999;
    t55096
}
