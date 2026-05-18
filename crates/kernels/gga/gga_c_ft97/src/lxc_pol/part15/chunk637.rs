//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 637/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk637<F: Float>(t1882: F, t4617: F, t4574: F, t4565: F, t4561: F, t4557: F, t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15899: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16255 = t1882 * t4617;
    let t16296 = t1882 * t4574;
    let t16298 = t1882 * t4565;
    let t16300 = t1882 * t4561;
    let t16302 = t1882 * t4557;
    let t16336 = F::new(2.0) / F::new(27.0) * t15606;
    let t16337 = F::new(2.0) / F::new(9.0) * t15609;
    let t16338 = t15612 / F::new(9.0);
    let t16342 = t15891 / F::new(3.0);
    let t16343 = F::new(2.0) / F::new(3.0) * t15894;
    let t16346 = F::new(2.0) / F::new(9.0) * t15899;
    (t16255, t16296, t16298, t16300, t16302, t16336, t16337, t16338, t16342, t16343, t16346)
}
