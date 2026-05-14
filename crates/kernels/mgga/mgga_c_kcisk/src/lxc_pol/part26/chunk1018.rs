//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1018/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1018<F: Float>(t27160: F, t487: F, t486: F, t21321: F, t6313: F, t14356: F, t8268: F, t27116: F, t27119: F, t27121: F, t27124: F, t27127: F, t27131: F, t27134: F, t27136: F, t27138: F, t27141: F, t27143: F, t27147: F, t27149: F, t27153: F, t27155: F, t27158: F) -> (F, F, F, F) {
    let t27161 = t487 * t27160;
    let t27162 = t486 * t27161;
    let t27164 = t21321 * t6313;
    let t27166 = t14356 * t8268;
    let t27168 = -t27116 / 16.0 + t27119 / 36.0 + 2.0 / 9.0 * t27121 - 3.0 / 8.0 * t27124 + t27127 / 6.0 + t27131 / 256.0 - t27134 / 64.0 + t27136 / 96.0 - t27138 / 192.0 + t27141 / 96.0 - t27143 / 72.0 - t27147 / 192.0 - t27149 / 192.0 - t27153 / 16.0 + t27155 / 24.0 + t27158 / 27.0 + t27162 / 24.0 - t27164 / 12.0 - t27166 / 128.0;
    (t27162, t27164, t27166, t27168)
}
