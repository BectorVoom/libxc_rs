//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1361/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1361<F: Float>(t4558: F, t7827: F, t11889: F, t2895: F, t4533: F, t7984: F, t7999: F, t10029: F, t10182: F, t1112: F, t1158: F, t1161: F, t11672: F, t11703: F, t11766: F, t12029: F, t28248: F, t28400: F, t2873: F, t28844: F, t2896: F, t2957: F, t32598: F, t33472: F, t4574: F, t4588: F, t4620: F, t505: F, t9761: F, t9850: F, t9856: F) -> (F, F, F, F, F) {
    let t33497 = t4558 * t7827;
    let t33504 = t2895 * t11889;
    let t33507 = t4533 * t7984;
    let t33510 = t4533 * t7999;
    let t33515 = 0.14222222222222222222e-2 * t9850 * t32598 - 0.1728e-1 * t28400 * t11703 * t9856 + 0.24192e-1 * t28844 * t33472 + 0.42666666666666666667e-2 * t10182 * t32598 - 12.0 * t1112 * t11672 - 6.0 * t505 * t4588 * t2873 - 8.0 * t10029 * t4574 + 0.768e-3 * t1161 * t33497 + 0.768e-6 * t11766 * t2896 + 1600.0 / 27.0 * t9761 * t12029 - 0.5376e-2 * t2957 * t33504 + 0.176e0 * t1158 * t33507 - 0.176e0 * t1161 * t33510 - 0.24e-1 * t28248 * t4620;
    (t33497, t33504, t33507, t33510, t33515)
}
