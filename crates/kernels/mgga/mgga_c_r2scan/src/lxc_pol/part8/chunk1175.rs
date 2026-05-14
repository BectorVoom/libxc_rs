//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1175/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1175<F: Float>(t18944: F, t21211: F, t40: F, t4911: F, t661: F, t5319: F, t732: F, t1654: F, t5325: F, t5305: F, t5261: F, t5300: F, t4889: F, t424: F, t5720: F, t21480: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22418 = t21211 * t18944 * t40;
    let t22424 = t4911 * t661;
    let t22426 = t732 * t5319;
    let t22428 = t1654 * t5325;
    let t22431 = t732 * t5305;
    let t22433 = t732 * t5261;
    let t22437 = t732 * t5300;
    let t22441 = t4889 * t661;
    let t22443 = t424 * t5720;
    let t22446 = 0.65061487801810439052e-1 * t61 * t21480;
    (t22418, t22424, t22426, t22428, t22431, t22433, t22437, t22441, t22443, t22446)
}
