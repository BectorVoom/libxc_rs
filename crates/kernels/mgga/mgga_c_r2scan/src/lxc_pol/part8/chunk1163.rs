//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1163/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1163<F: Float>(t18852: F, t229: F, t60: F, t4889: F, t726: F, t21813: F, t219: F, t225: F, t234: F, t21483: F, t61: F, t22006: F, t5438: F, t5348: F, t732: F, t5352: F) -> (F, F, F, F, F, F, F) {
    let t22105 = 24.0 * t18852 * t60 * t229;
    let t22107 = 480.0 * t4889 * t726;
    let t22114 = 0.5848223622634646207e0 * t234 * t219 * t21813 * t225;
    let t22116 = 0.13689115175718902887e4 * t61 * t21483;
    let t22125 = 0.23422135608651758058e1 * t5438 * t22006;
    let t22130 = t732 * t5348;
    let t22132 = t732 * t5352;
    (t22105, t22107, t22114, t22116, t22125, t22130, t22132)
}
