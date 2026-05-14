//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1274/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1274<F: Float>(t23917: F, t4889: F, t899: F, t5006: F, t963: F, t1234: F, t2266: F, t2867: F, t795: F, t19425: F, t19427: F, t19429: F, t2858: F, t7016: F, t2859: F, t4933: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23918 = 240.0 * t23917;
    let t23919 = t4889 * t899;
    let t23920 = 120.0 * t23919;
    let t23921 = t963 * t5006;
    let t23922 = 0.10254018858216406658e4 * t23921;
    let t23926 = 9.0 * t2266 * t2867 * t1234 * t795;
    let t23927 = 0.30762056574649219973e4 * t19425;
    let t23928 = 0.10526802520742363173e2 * t19427;
    let t23929 = 180.0 * t19429;
    let t23932 = 18.0 * t2858 * t7016 * t1234;
    let t23935 = 6.0 * t2858 * t2859 * t4933;
    (t23918, t23920, t23922, t23926, t23927, t23928, t23929, t23932, t23935)
}
