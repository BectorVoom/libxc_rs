//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1265/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1265<F: Float>(t2271: F, t7133: F, t4959: F, t955: F, t1376: F, t2461: F, t19026: F, t986: F, t1048: F, t6894: F, t5002: F, t963: F, t1422: F, t2452: F, t2321: F, t19396: F) -> (F, F, F, F, F, F, F, F) {
    let t23783 = t2271 * t7133;
    let t23785 = t4959 * t955;
    let t23788 = t1376 * t2461;
    let t23791 = t986 * t19026;
    let t23794 = 6.0 * t1048 * t23791 * t6894;
    let t23795 = t963 * t5002;
    let t23796 = 0.5848223622634646207e0 * t23795;
    let t23797 = t1422 * t2452;
    let t23798 = 96.0 * t23797;
    let t23799 = t2321 * t2461;
    let t23800 = 3.0 * t23799;
    let t23801 = 0.18311447306006545054e-3 * t19396;
    (t23783, t23785, t23788, t23794, t23796, t23798, t23800, t23801)
}
