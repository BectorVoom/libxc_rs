//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 846/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk846<F: Float>(t10078: F, t7591: F, t941: F, t10075: F, t2902: F, t761: F, t3221: F, t1474: F, t277: F, t1051: F, t2043: F, t6808: F) -> (F, F, F, F, F, F, F, F) {
    let t10079 = t7591 * t941 * t10078;
    let t10080 = t10075 * t10079;
    let t10102 = t2902 * t761;
    let t10103 = t10102 * t3221;
    let t10105 = t1474 * t277;
    let t10106 = t10105 * t3221;
    let t10108 = t2043 * t1051;
    let t10110 = t2902 * t6808;
    (t10079, t10080, t10102, t10103, t10105, t10106, t10108, t10110)
}
