//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1072/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1072<F: Float>(t18903: F, t18904: F, t234: F, t4862: F, t405: F, t4888: F, t89: F, t1059: F, t37: F, t453: F, t4816: F, t458: F, t4889: F, t1379: F, t1383: F, t468: F, t5385: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18908 = 0.12304822629859687989e5 * t234 * t18903 * t18904 * t4862;
    let t18911 = t405 * t4888;
    let t18912 = t18911 * t89;
    let t18914 = t37 * t1059;
    let t18916 = 840.0 * t18914 * t89;
    let t18920 = 0.14035736694323150897e2 * t234 * t4816 * t18904 * t453;
    let t18922 = 480.0 * t4889 * t458;
    let t18923 = t1379 * t1379;
    let t18924 = 1.0 / t18923;
    let t18926 = t1383 * t1383;
    let t18927 = 1.0 / t18926;
    let t18930 = 0.91082604192152556044e5 * t234 * t18924 * t18904 * t18927;
    let t18931 = t5385 * t468;
    (t18908, t18911, t18912, t18914, t18916, t18920, t18922, t18924, t18927, t18930, t18931)
}
