//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 933/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk933<F: Float>(t10810: F, t2150: F, t574: F, t1266: F, t507: F, t512: F, t260: F, t259: F, t277: F, t254: F, t3316: F, t776: F) -> (F, F, F, F, F, F, F) {
    let t10811 = t10810 * t2150;
    let t10812 = t574 * t10811;
    let t10813 = F::cast_from(0.23115257973478049502e0_f64) * t10812;
    let t10818 = t512 * t1266 * t507;
    let t10831 = t260 * t1266;
    let t10833 = t259 * t10831 * t277;
    let t10834 = t254 * t10833;
    let t10839 = t776 * t3316;
    (t10811, t10813, t10818, t10831, t10833, t10834, t10839)
}
