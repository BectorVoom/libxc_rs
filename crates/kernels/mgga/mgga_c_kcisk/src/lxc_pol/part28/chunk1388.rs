//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1388/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1388<F: Float>(t10798: F, t33031: F, t35118: F, t1863: F, t35248: F, t415: F, t35092: F, t5074: F, t1799: F, t24061: F, t9679: F, t1869: F, t22995: F, t34159: F, t23000: F, t33017: F) -> (F, F, F, F, F, F) {
    let t121960 = t33031 * t10798 * t35118;
    let t121971 = t415 * t1863 * t35248;
    let t121973 = t5074 * t35092;
    let t121976 = t1799 * t9679 * t24061;
    let t121979 = t1869 * t34159 * t22995;
    let t121982 = t1869 * t33017 * t23000;
    (t121960, t121971, t121973, t121976, t121979, t121982)
}
