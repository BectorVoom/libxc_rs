//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 276/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk276<F: Float>(t1043: F, t1046: F, t1014: F, t1020: F, t1024: F, t1028: F, t1041: F) -> (F,) {
    let t1047 = t1043 * t1046;
    let t1049 = 0.13900948042322754167e-2 * t1014 + 0.10120768229166666667e-4 * t1020 - 0.86880925264517213544e-4 * t1024 - 0.11594181388521408695e-4 * t1028 - 0.84412963981222021454e-7 * t1041 + 0.72463633678258804342e-6 * t1047;
    (t1049,)
}
