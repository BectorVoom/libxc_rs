//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 859/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk859<F: Float>(t3774: F, t862: F, t1197: F, t2296: F, t2318: F, t3083: F, t3103: F, t3116: F, t3121: F, t3136: F, t3140: F, t365: F, t3780: F, t6272: F, t6323: F, t8071: F, t8107: F, t8115: F, t872: F, t9866: F, t9870: F, t9875: F, t9878: F, t9881: F, t9888: F) -> (F, F) {
    let t9891 = t3774 * t862;
    let t9902 = -t9866 - t9870 - 0.23392894490538584828e1 * t8071 * t3121 + 0.34631718211362927517e2 * t8107 * t3140 + 0.35089341735807877242e1 * t2318 * t9875 - 0.23392894490538584828e1 * t2296 * t9878 - 0.10389515463408878255e3 * t6323 * t9881 - 0.310907e-1 * t9888 * t365 + 1.0 * t9891 * t872 + 2.0 * t8115 * t1197 + 2.0 * t3083 * t3103 - 2.0 * t6272 * t3780 + 0.11696447245269292414e1 * t3116 * t3136;
    (t9891, t9902)
}
