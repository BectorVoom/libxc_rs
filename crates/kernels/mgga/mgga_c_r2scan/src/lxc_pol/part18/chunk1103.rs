//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1103/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1103<F: Float>(t37580: F, t40556: F, t40560: F, t40564: F, t40587: F, t43780: F, t43782: F, t43783: F, t43785: F, t43787: F, t43789: F, t43791: F, t43795: F, t43797: F, t43921: F, t11479: F, t11497: F, t3262: F) -> (F, F) {
    let t43924 = t43780 - t43782 - t43783 + 0.16260079888840015101e-2 * t40556 + t40560 + t43785 + t43787 - 0.86737941314158990623e-4 * t40564 + t43789 + 0.34200192530023447503e-6 * t37580 - t43791 + 0.19211284388664477842e-2 * t43921 - 0.1616301098968908129e-5 * t40587 - t43795 - t43797;
    let t43929 = 3.0 / 2.0 * t3262 * t11479 * t11497;
    (t43924, t43929)
}
