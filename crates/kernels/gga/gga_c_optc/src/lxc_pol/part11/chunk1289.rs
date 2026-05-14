//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1289/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1289<F: Float>(t1188: F, t14849: F, t15083: F, t17516: F, t18191: F, t277: F, t3245: F, t4281: F, t4290: F, t5229: F, t52329: F, t53399: F, t53851: F, t58322: F, t58917: F, t59086: F, t59088: F, t59152: F, t59154: F, t60243: F, t95: F) -> (F,) {
    let t60249 = 800.0 / 81.0 * t14849 * t18191 + 136400.0 / 729.0 * t53399 * t5229 + 200.0 / 3.0 * t15083 * t17516 + 6.0 * t4281 * t3245 * t4290 * t58322 + 16000000.0 / 729.0 * t52329 * t58917 + 0.25844881434903430496e-2 * t95 * t277 * t60243 * t1188 + 200.0 / 27.0 * t53851 - t59086 + t59088 + t59152 + t59154;
    (t60249,)
}
