//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1137/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1137<F: Float>(t30258: F, t587: F, t912: F, t21071: F, t901: F, t4379: F, t9573: F, t1402: F, t1429: F, t3162: F, t20237: F, t544: F) -> (F, F, F, F, F) {
    let t30260 = t587 * t912 * t30258;
    let t30263 = F::cast_from(0.29792074959875355558e-1_f64) * t21071 * t901;
    let t30265 = F::cast_from(0.59584149919750711116e-1_f64) * t4379 * t9573;
    let t30288 = F::cast_from(0.17875244975925213335e0_f64) * t1429 * t1402 * t3162;
    let t30292 = t544 * t20237;
    (t30260, t30263, t30265, t30288, t30292)
}
