//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 828/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk828(t13848: f64, t13850: f64, t8688: f64, t2314: f64, t68658: f64, t14363: f64, t15231: f64, t13996: f64, t2868: f64, t11723: f64, t69041: f64, t14236: f64, t2078: f64, t3369: f64, t56399: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74861 = t8688 * t13848 * t13850;
    let t74864 = t2314 * t68658 * t13850;
    let t74867 = t14363 * t15231;
    let t74870 = 0.2993560425465952141e-1_f64 * t2868 * t13996;
    let t74873 = t69041 * t11723;
    let t74889 = t14236 * t3369 * t2078 * t56399;
    (t74861, t74864, t74867, t74870, t74873, t74889)
}
