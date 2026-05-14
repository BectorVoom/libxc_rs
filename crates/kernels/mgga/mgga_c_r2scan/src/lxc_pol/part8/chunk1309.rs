//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1309/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1309<F: Float>(t18869: F, t18872: F, t18875: F, t18878: F, t18888: F, t18894: F, t23708: F, t32078: F, t32082: F, t32086: F, t32087: F, t32088: F, t10288: F, t2271: F, t2483: F, t3128: F) -> (F, F, F) {
    let t32103 = t32078 + t18869 - t18872 - t18875 - t18878 - t32082 + t32086 - t32087 - t18888 - t32088 - t18894 + t23708;
    let t32106 = t2271 * t10288;
    let t32108 = t2483 * t3128;
    (t32103, t32106, t32108)
}
