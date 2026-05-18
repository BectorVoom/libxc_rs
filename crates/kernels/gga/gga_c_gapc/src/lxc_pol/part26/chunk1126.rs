//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1126/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1126<F: Float>(t17760: F, t2580: F, t33273: F, t1: F, t33549: F, t128: F, t18639: F, t941: F, t2660: F, t24759: F, t667: F, t277: F) -> (F, F, F, F, F, F) {
    let t33956 = t17760 * t33273 * t2580;
    let t33958 = t33549 * t1;
    let t33961 = t18639 * t941 * t128;
    let t33962 = t2660 * t33958 * t33961;
    let t33965 = t667 * t24759 * M_PI;
    let t33966 = t277 * t33965;
    (t33956, t33958, t33961, t33962, t33965, t33966)
}
