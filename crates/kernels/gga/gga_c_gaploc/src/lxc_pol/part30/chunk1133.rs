//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1133/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1133<F: Float>(t30258: F, t587: F, t912: F, t21071: F, t901: F, t4379: F, t9573: F, t1402: F, t1429: F, t3162: F, t20237: F, t544: F) -> (F, F, F, F, F) {
    let t30260 = t587 * t912 * t30258;
    let t30261 = F::new(0.38342925953920749676e0) * t30260;
    let t30263 = F::new(0.29792074959875355558e-1) * t21071 * t901;
    let t30265 = F::new(0.59584149919750711116e-1) * t4379 * t9573;
    let t30288 = F::new(0.17875244975925213335e0) * t1429 * t1402 * t3162;
    let t30292 = t544 * t20237;
    (t30261, t30263, t30265, t30288, t30292)
}
