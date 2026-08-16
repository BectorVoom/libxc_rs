//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1255/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1255(t10935: f64, t3165: f64, t3446: f64, t37580: f64, t40556: f64, t40560: f64, t40564: f64, t40587: f64, t43780: f64, t43782: f64, t43783: f64, t43785: f64, t43787: f64, t43789: f64, t43791: f64, t43795: f64, t43797: f64) -> f64 {
    let t43921 = t3446 * t10935 * t3165;
    let t43924 = t43780 - t43782 - t43783 + 0.16260079888840015101e-2_f64 * t40556 + t40560 + t43785 + t43787 - 0.86737941314158990623e-4_f64 * t40564 + t43789 + 0.34200192530023447503e-6_f64 * t37580 - t43791 + 0.19211284388664477842e-2_f64 * t43921 - 0.1616301098968908129e-5_f64 * t40587 - t43795 - t43797;
    t43924
}
