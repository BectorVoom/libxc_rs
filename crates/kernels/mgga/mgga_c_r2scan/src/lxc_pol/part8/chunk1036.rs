//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1036/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1036<F: Float>(t5754: F, t5761: F, t5766: F, t5770: F, t5774: F, t5777: F, t5793: F, t5812: F, t5907: F, t5919: F, t5923: F, t7825: F, t7827: F, t7832: F, t8990: F, t8995: F, t8998: F, t9010: F) -> (F,) {
    let t10263 = 0.8103123984e0 * t8990 - t5754 + t5907 - 0.1200612870296e-1 * t8995 - 0.60030643514799999999e-2 * t8998 + t5761 + t5766 + t5770 - t5774 + t5919 - t5777 - 0.96319466275353142157e0 * t7825 + 0.65061487801810439052e-1 * t7827 - t5793 + 0.80040858019733333331e-2 * t7832 + t5923 - 0.1714584e0 * t9010 + t5812;
    (t10263,)
}
