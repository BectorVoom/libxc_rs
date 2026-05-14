//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 644/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk644<F: Float>(t1250: F, t8232: F, t1882: F, t4164: F, t4169: F, t12001: F, t4159: F, t4241: F, t681: F, t89: F, t1240: F, t2770: F) -> (F, F, F, F, F, F) {
    let t15147 = t8232 * t1250;
    let t15168 = 4.0 / 9.0 * t1882 * t4164;
    let t15170 = 2.0 / 9.0 * t1882 * t4169;
    let t15180 = t12001 * t4159;
    let t15190 = 2.0 / 9.0 * t89 * t681 * t4241;
    let t15191 = t2770 * t1240;
    (t15147, t15168, t15170, t15180, t15190, t15191)
}
