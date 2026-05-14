//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1085/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1085<F: Float>(t5006: F, t732: F, t4990: F, t1416: F, t1512: F, t607: F, t6880: F, t58: F, t6879: F, t766: F, t2271: F, t6014: F, t6017: F, t2065: F, t6887: F, t1783: F, t2321: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19425 = t732 * t5006;
    let t19427 = t732 * t4990;
    let t19429 = t1416 * t1512;
    let t19439 = t6880 * t607;
    let t19444 = t6879 * t58;
    let t19445 = t19444 * t766;
    let t19447 = t2271 * t6014;
    let t19455 = t2271 * t6017;
    let t19457 = t6887 * t2065;
    let t19464 = t2321 * t1783;
    (t19425, t19427, t19429, t19439, t19444, t19445, t19447, t19455, t19457, t19464)
}
