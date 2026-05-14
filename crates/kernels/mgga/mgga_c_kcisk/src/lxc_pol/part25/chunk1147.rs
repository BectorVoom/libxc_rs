//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1147/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1147<F: Float>(t2815: F, t5533: F, t12345: F, t12352: F, t33067: F, t33070: F, t33074: F, t33075: F, t33079: F, t33082: F, t33085: F, t33088: F, t5552: F, t9760: F, t9763: F, t240: F, t33073: F, t33077: F, t33131: F, t33309: F) -> (F, F) {
    let t33312 = t2815 * t5533;
    let t33316 = 4.0 * t12345 * t9763 - 6.0 * t12352 * t33312 - t5552 * t9760 - t33067 + t33070 + t33074 + t33075 + t33079 + t33082 - t33085 - t33088;
    let t33319 = t33067 - t33070 + t33073 - t33074 - t33075 + t33077 - t33079 - t33082 + t33085 + t33088 - t33131 + t240 * (t33309 + t33316);
    (t33312, t33319)
}
