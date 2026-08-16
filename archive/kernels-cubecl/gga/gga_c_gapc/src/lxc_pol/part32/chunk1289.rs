//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1289/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1289<F: Float>(t10357: F, t11674: F, t35734: F, t190: F, t24086: F, t35729: F, t6852: F, t10256: F, t11663: F, t2229: F, t3729: F, t11670: F, t828: F) -> (F, F, F, F, F) {
    let t35861 = t35734 * t11674 * t10357;
    let t35865 = t35729 * t6852 * t190 * t24086;
    let t35867 = t10256 * t11663;
    let t35869 = t2229 * t3729;
    let t35871 = t828 * t11670;
    (t35861, t35865, t35867, t35869, t35871)
}
