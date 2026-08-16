//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 633/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk633<F: Float>(t242: F, t27890: F, t28024: F, t27934: F, t24569: F, t3875: F, t10007: F, t3880: F, t14175: F, t1882: F, t6942: F, t6867: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28195 = t242 * t27890;
    let t28198 = t242 * t28024;
    let t28201 = t242 * t27934;
    let t28204 = t24569 * t3875;
    let t28205 = t10007 * t28204;
    let t28208 = t24569 * t3880;
    let t28209 = t14175 * t28208;
    let t28212 = t1882 * t6942;
    let t28214 = t1882 * t6867;
    (t28195, t28198, t28201, t28204, t28205, t28208, t28209, t28212, t28214)
}
