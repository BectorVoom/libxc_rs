//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 795/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk795<F: Float>(t7007: F, t76: F, t5896: F, t2810: F, t595: F, t637: F, t2813: F, t236: F, t5709: F, t5727: F, t5736: F, t5739: F, t5889: F, t5891: F, t5895: F, t7802: F) -> (F,) {
    let t7803 = t7007 * t76;
    let t7807 = 32.0 * t5896;
    let t7808 = t595 * t2810;
    let t7810 = 0.40020429009866666666e-2 * t7808 * t637;
    let t7811 = t595 * t2813;
    let t7813 = 0.40020429009866666666e-2 * t7811 * t637;
    let t7814 = t7802 + 0.5848223622634646207e0 * t7803 * t236 - t5889 - t5709 + t5727 - t5736 - t5739 - 2.0 * t5891 - t5895 + t7807 - t7810 - t7813;
    (t7814,)
}
