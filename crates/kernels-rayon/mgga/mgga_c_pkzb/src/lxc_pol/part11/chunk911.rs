//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 911/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk911(t6090: f64, t6348: f64, t7955: f64, t8225: f64, t9782: f64, t9797: f64, t3774: f64, t862: f64, t1197: f64, t2296: f64, t2318: f64, t3083: f64, t3103: f64, t3116: f64, t3121: f64, t3136: f64, t3140: f64, t365: f64, t3780: f64, t6272: f64, t6323: f64, t8071: f64, t8107: f64, t8115: f64, t872: f64, t9866: f64, t9870: f64, t9875: f64, t9878: f64, t9881: f64) -> (f64, f64, f64) {
    let t9888 = -t6348 + 0.22831111111111111111e-1_f64 * t6090 + 0.45662222222222222221e-1_f64 * t7955 - t8225 - 0.17123333333333333333e-1_f64 * t9782 + 0.5137e-1_f64 * t9797;
    let t9891 = t3774 * t862;
    let t9902 = -t9866 - t9870 - 0.23392894490538584828e1_f64 * t8071 * t3121 + 0.34631718211362927517e2_f64 * t8107 * t3140 + 0.35089341735807877242e1_f64 * t2318 * t9875 - 0.23392894490538584828e1_f64 * t2296 * t9878 - 0.10389515463408878255e3_f64 * t6323 * t9881 - 0.310907e-1_f64 * t9888 * t365 + 1.0_f64 * t9891 * t872 + 2.0_f64 * t8115 * t1197 + 2.0_f64 * t3083 * t3103 - 2.0_f64 * t6272 * t3780 + 0.11696447245269292414e1_f64 * t3116 * t3136;
    (t9888, t9891, t9902)
}
