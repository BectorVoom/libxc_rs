//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1182/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1182<F: Float>(t11365: F, t7294: F, t7880: F, t11897: F, t9670: F, t10058: F, t11808: F, t28472: F, t3708: F, t9574: F, t11840: F, t9520: F) -> (F, F, F, F, F) {
    let t33770 = t7294 * t11365 * t7880;
    let t33772 = t11897 * t9670;
    let t33774 = t11808 * t10058;
    let t33777 = t9574 * t3708 * t28472;
    let t33779 = t11840 * t9520;
    (t33770, t33772, t33774, t33777, t33779)
}
