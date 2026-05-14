//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1129/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1129<F: Float>(t1691: F, t1719: F, t5537: F, t5538: F, t61: F, t1898: F, t5686: F, t650: F, t189: F, t21062: F, t631: F, t5317: F, t695: F, t226: F, t5865: F, t5455: F, t721: F) -> (F, F, F, F, F, F, F) {
    let t21234 = t1691 * t1719;
    let t21237 = 0.54649562515291533626e6 * t61 * t5537 * t5538 * t21234;
    let t21244 = 0.64327917994770140268e2 * t650 * t1898 * t5686;
    let t21247 = 0.12822e1 * t631 * t21062 * t189;
    let t21248 = t5317 * t695;
    let t21251 = 0.14035736694323150897e2 * t5865 * t226 * t21248;
    let t21254 = 0.41558061853635513021e3 * t5455 * t721 * t21248;
    (t21234, t21237, t21244, t21247, t21248, t21251, t21254)
}
