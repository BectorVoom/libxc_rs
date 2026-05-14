//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1035/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1035<F: Float>(t10245: F, t236: F, t5678: F, t5682: F, t5689: F, t5709: F, t5727: F, t5736: F, t5739: F, t5886: F, t5889: F, t5895: F, t5897: F, t5901: F, t7798: F, t7817: F, t8893: F, t8970: F, t8988: F, t951: F) -> (F,) {
    let t10254 = -t5678 - t5682 - t5689 + 0.5848223622634646207e0 * t10245 * t236 - t5886 + 0.17544670867903938621e1 * t7798 - t5889 - t5709 + 0.17544670867903938621e1 * t8970 - 0.2025780996e0 * t951 * t8893 + t5727 - t5736 - t5739 - t5895 - t5897 - 0.300153217574e-2 * t7817 + t5901 + 0.4051561992e0 * t8988;
    (t10254,)
}
