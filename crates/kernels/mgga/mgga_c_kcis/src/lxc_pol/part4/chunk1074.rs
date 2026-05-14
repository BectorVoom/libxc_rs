//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1074/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1074<F: Float>(t25: F, t5337: F, t1251: F, t10990: F, t10993: F, t10996: F, t11009: F, t11014: F, t11086: F, t15473: F, t15477: F, t15482: F, t15487: F, t15493: F, t1853: F, t3490: F, t3514: F, t5307: F, t5311: F, t5338: F) -> (F,) {
    let t15494 = t25 * t5337;
    let t15496 = t1251 * t15494 / 288.0;
    let t15499 = t11086 * t5307 / 108.0 + t11086 * t5311 / 54.0 + t10990 / 432.0 - t10993 / 576.0 - t1251 * t15473 / 192.0 + t15477 / 864.0 + t3490 * t5338 / 36.0 + t3514 * t15482 / 144.0 + t1251 * t15487 / 288.0 - 11.0 / 216.0 * t10996 * t1853 + t15493 - t15496 + t11009 / 108.0 + t11014 / 288.0;
    (t15499,)
}
