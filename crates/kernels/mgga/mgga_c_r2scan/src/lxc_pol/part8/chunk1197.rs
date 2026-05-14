//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1197/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1197<F: Float>(t2788: F, t4970: F, t4994: F, t963: F, t1422: F, t2484: F, t1524: F, t2747: F, t1416: F, t2452: F, t4885: F, t899: F, t4889: F, t5006: F, t2813: F, t6887: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23901 = t2788 * t4970;
    let t23903 = t963 * t4994;
    let t23906 = 96.0 * t1422 * t2484;
    let t23909 = t2747 * t1524;
    let t23910 = 0.35089341735807877242e1 * t23909;
    let t23915 = t1416 * t2452;
    let t23917 = t4885 * t899;
    let t23918 = 240.0 * t23917;
    let t23919 = t4889 * t899;
    let t23921 = t963 * t5006;
    let t23939 = t6887 * t2813;
    (t23901, t23903, t23906, t23910, t23915, t23918, t23919, t23921, t23939)
}
