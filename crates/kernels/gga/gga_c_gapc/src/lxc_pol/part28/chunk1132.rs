//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1132/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1132<F: Float>(t17874: F, t35382: F, t35766: F, t10237: F, t3729: F, t11670: F, t3235: F, t11679: F, t24092: F, t6182: F, t9497: F, t10346: F, t134: F, t6939: F, t11210: F, t11657: F, t7557: F) -> (F, F, F, F, F, F, F) {
    let t35881 = t35766 * t35382 * t17874;
    let t35883 = t10237 * t3729;
    let t35885 = t3235 * t11670;
    let t35890 = t11679 * t24092;
    let t35894 = t6182 * t9497;
    let t35895 = t10346 * t6939 * t134 * t35894;
    let t35898 = t11657 * t11210 * t7557;
    (t35881, t35883, t35885, t35890, t35894, t35895, t35898)
}
