//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 723/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk723<F: Float>(t15425: F, t2862: F, t319: F, t14608: F, t296: F, t1248: F, t2739: F, t840: F, t871: F, t15131: F, t18: F, t875: F, t2882: F, t2881: F, t14116: F, t4265: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15427 = t2862 * t319 * t15425;
    let t15430 = t296 * t14608;
    let t15433 = t1248 * t2739;
    let t15435 = t840 * t871 * t15433;
    let t15438 = t296 * t15131;
    let t15441 = t18 * t875;
    let t15442 = t2882 * t15441;
    let t15443 = t2881 * t15442;
    let t15446 = t4265 * t14116;
    (t15427, t15430, t15433, t15435, t15438, t15441, t15442, t15443, t15446)
}
