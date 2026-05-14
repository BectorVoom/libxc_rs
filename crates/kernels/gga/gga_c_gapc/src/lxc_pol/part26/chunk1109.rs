//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1109/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1109<F: Float>(t190: F, t24086: F, t35729: F, t6852: F, t10256: F, t11663: F, t2229: F, t3729: F, t11670: F, t828: F, t10346: F, t134: F, t2207: F, t35834: F, t10301: F, t2580: F, t9497: F) -> (F, F, F, F, F, F) {
    let t35865 = t35729 * t6852 * t190 * t24086;
    let t35867 = t10256 * t11663;
    let t35869 = t2229 * t3729;
    let t35871 = t828 * t11670;
    let t35875 = t10346 * t2207 * t134 * t35834;
    let t35878 = t10301 * t2580 * t9497;
    (t35865, t35867, t35869, t35871, t35875, t35878)
}
