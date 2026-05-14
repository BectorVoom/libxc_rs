//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1293/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1293<F: Float>(t109501: F, t109533: F, t109535: F, t109556: F, t1173: F, t121882: F, t121928: F, t121983: F, t122031: F, t122077: F, t122116: F, t122173: F, t122227: F, t122273: F, t122321: F, t122361: F, t122408: F, t122458: F, t122504: F, t122550: F, t122591: F, t122624: F, t122657: F, t124358: F, t124401: F, t124432: F, t124476: F, t124690: F, t124737: F, t124777: F, t124832: F, t124876: F, t124914: F, t124931: F, t124970: F, t125010: F, t125050: F, t125087: F, t1403: F, t1454: F, t17715: F, t193: F, t247: F, t27882: F, t27894: F, t27906: F, t27963: F, t30896: F, t6011: F, t6754: F, t6844: F, t96818: F) -> (F,) {
    let t125105 = -2.0 / 3.0 * t1403 * t193 * t27882 * t27963 - 4.0 / 27.0 * t96818 - 4.0 * t121882 - t109501 - t17715 * t1454 - t247 * (t124737 + t122227 + t122458 + t124914 + t125087 + t122624 + t122077 + t122504 + t121983 + t124832 + t121928 + t122321 + t125050 + t122273 + t122116 + t124432 + t124970 + t125010 + t122361 + t122591 + t124931 + t122657 + t124876 + t124401 + t122173 + t122550 + t124690 + t124358 + t122408 + t124476 + t122031 + t124777) + t27894 * t6844 / 3.0 - t109533 - t109535 - 2.0 / 3.0 * t27894 * t6754 - 8.0 / 27.0 * t109556 + t1403 * t193 * t27906 * t1173 / 3.0 - t30896 * t6011 / 3.0;
    (t125105,)
}
