//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 856/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk856<F: Float>(t2896: F, t43: F, t47: F, t2908: F, t50: F, t52: F, t3901: F, t872: F, t3909: F, t852: F, t180: F, t3645: F) -> (F, F, F, F, F) {
    let t12161 = F::cast_from(1.0_f64) / t47 / t2896 / t43;
    let t12177 = F::cast_from(1.0_f64) / t52 / t2908 / t50;
    let t12196 = t3901 * t872;
    let t12198 = t852 * t3909;
    let t12200 = t3645 * t180;
    (t12161, t12177, t12196, t12198, t12200)
}
