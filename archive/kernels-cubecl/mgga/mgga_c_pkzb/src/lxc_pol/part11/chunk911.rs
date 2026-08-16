//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 911/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk911<F: Float>(t6090: F, t6348: F, t7955: F, t8225: F, t9782: F, t9797: F, t3774: F, t862: F, t1197: F, t2296: F, t2318: F, t3083: F, t3103: F, t3116: F, t3121: F, t3136: F, t3140: F, t365: F, t3780: F, t6272: F, t6323: F, t8071: F, t8107: F, t8115: F, t872: F, t9866: F, t9870: F, t9875: F, t9878: F, t9881: F) -> (F, F, F) {
    let t9888 = -t6348 + F::cast_from(0.22831111111111111111e-1_f64) * t6090 + F::cast_from(0.45662222222222222221e-1_f64) * t7955 - t8225 - F::cast_from(0.17123333333333333333e-1_f64) * t9782 + F::cast_from(0.5137e-1_f64) * t9797;
    let t9891 = t3774 * t862;
    let t9902 = -t9866 - t9870 - F::cast_from(0.23392894490538584828e1_f64) * t8071 * t3121 + F::cast_from(0.34631718211362927517e2_f64) * t8107 * t3140 + F::cast_from(0.35089341735807877242e1_f64) * t2318 * t9875 - F::cast_from(0.23392894490538584828e1_f64) * t2296 * t9878 - F::cast_from(0.10389515463408878255e3_f64) * t6323 * t9881 - F::cast_from(0.310907e-1_f64) * t9888 * t365 + F::cast_from(1.0_f64) * t9891 * t872 + F::cast_from(2.0_f64) * t8115 * t1197 + F::cast_from(2.0_f64) * t3083 * t3103 - F::cast_from(2.0_f64) * t6272 * t3780 + F::cast_from(0.11696447245269292414e1_f64) * t3116 * t3136;
    (t9888, t9891, t9902)
}
