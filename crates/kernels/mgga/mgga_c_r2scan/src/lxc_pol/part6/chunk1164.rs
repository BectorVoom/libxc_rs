//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1164/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1164<F: Float>(t21234: F, t5537: F, t5538: F, t61: F, t159: F, t585: F, t5879: F, t617: F, t1898: F, t5686: F, t650: F, t189: F, t21062: F, t631: F, t5317: F, t695: F) -> (F, F, F, F, F) {
    let t21237 = 0.54649562515291533626e6 * t61 * t5537 * t5538 * t21234;
    let t21240 = t159 * t5879 * t585 * t617;
    let t21244 = 0.64327917994770140268e2 * t650 * t1898 * t5686;
    let t21247 = 0.12822e1 * t631 * t21062 * t189;
    let t21248 = t5317 * t695;
    (t21237, t21240, t21244, t21247, t21248)
}
