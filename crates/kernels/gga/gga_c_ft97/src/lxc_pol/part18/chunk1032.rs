//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1032/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1032<F: Float>(t1969: F, t27165: F, t379: F, t5899: F, t375: F, t6681: F, t89: F, t1017: F, t23925: F, t28: F, t376: F, t6677: F, t26791: F, t558: F, t3408: F, t5778: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27167 = t1969 * t27165 * t379;
    let t27168 = t5899 * t27167;
    let t27171 = t89 * t375 * t6681;
    let t27174 = t23925 * t1017;
    let t27175 = t28 * t27174;
    let t27176 = t89 * t27175;
    let t27178 = t376 * t6677;
    let t27179 = t89 * t27178;
    let t27181 = t26791 * t558;
    let t27182 = t28 * t27181;
    let t27183 = t89 * t27182;
    let t27185 = t5778 * t3408;
    (t27167, t27168, t27171, t27174, t27176, t27179, t27181, t27183, t27185)
}
