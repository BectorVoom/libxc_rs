//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 990/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk990<F: Float>(t19329: F, t296: F, t5374: F, t870: F, t875: F, t4635: F, t824: F, t2875: F, t2874: F, t1882: F, t5315: F, t1248: F, t15133: F) -> (F, F, F, F, F, F) {
    let t19330 = t296 * t19329;
    let t19333 = t5374 * t870;
    let t19334 = t19333 * t875;
    let t19335 = t296 * t19334;
    let t19338 = t4635 * t824;
    let t19339 = t2875 * t19338;
    let t19340 = t2874 * t19339;
    let t19343 = t1882 * t5315;
    let t19345 = t15133 * t1248;
    (t19330, t19334, t19335, t19340, t19343, t19345)
}
