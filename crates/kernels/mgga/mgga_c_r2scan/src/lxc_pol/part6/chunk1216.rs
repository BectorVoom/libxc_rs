//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1216/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1216<F: Float>(t4889: F, t661: F, t424: F, t5720: F, t21480: F, t61: F, t1762: F, t1767: F, t5664: F, t1771: F, t5418: F, t406: F, t5890: F, t5714: F, t1419: F, t1789: F) -> (F, F, F, F, F, F, F, F) {
    let t22441 = t4889 * t661;
    let t22443 = t424 * t5720;
    let t22446 = 0.65061487801810439052e-1 * t61 * t21480;
    let t22449 = 0.64212977516902094772e0 * t1762 * t1767 * t5664;
    let t22450 = t1771 * t5418;
    let t22452 = t406 * t5890;
    let t22454 = t424 * t5714;
    let t22457 = t1419 * t1789;
    (t22441, t22443, t22446, t22449, t22450, t22452, t22454, t22457)
}
