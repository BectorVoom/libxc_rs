//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1046/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1046<F: Float>(t10485: F, t10683: F, t10703: F, t1091: F, t1255: F, t1901: F, t21978: F, t22208: F, t22212: F, t2881: F, t44204: F, t44518: F, t446: F, t4965: F, t5330: F, t5414: F, t72391: F, t84283: F, t84312: F, t84317: F, t84581: F, t90603: F, t90717: F) -> (F,) {
    let t90765 = -4.0 / 9.0 * t84283 + 4.0 / 3.0 * t1901 * t72391 * t5414 + 4.0 / 9.0 * t1901 * t2881 * t84581 * t1091 - 8.0 * t446 * t10683 * t1255 * t21978 + 8.0 / 3.0 * t1901 * t2881 * t44204 * t90717 + 8.0 / 3.0 * t1901 * t2881 * t10485 * t90603 - 4.0 / 9.0 * t84312 - 8.0 / 3.0 * t84317 - 8.0 / 9.0 * t1901 * t44518 * t5330 * t4965 - 4.0 / 3.0 * t1901 * t10703 * t22208 * t1091 - 4.0 / 3.0 * t1901 * t10703 * t22212 * t1091;
    (t90765,)
}
