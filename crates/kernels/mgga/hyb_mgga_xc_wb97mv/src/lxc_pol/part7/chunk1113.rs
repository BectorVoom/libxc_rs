//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1113/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1113<F: Float>(t1137: F, t11889: F, t10080: F, t3809: F, t2848: F, t535: F, t3813: F, t1142: F, t4077: F, t9838: F, t2856: F, t516: F, t396: F, t8020: F, t4083: F, t646: F, sigma0: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11890 = t11889 * t1137;
    let t11893 = t10080 * t3809;
    let t11896 = t2848 * sigma2;
    let t11897 = t535 * t11896;
    let t11898 = t10080 * t3813;
    let t11901 = t4077 * t1142;
    let t11902 = t11901 * t1137;
    let t11909 = t535 * t9838;
    let t11922 = t2856 * sigma2;
    let t11923 = t516 * t11922;
    let t11928 = t8020 * t396;
    let t11929 = t4083 * sigma0;
    let t11930 = t11929 * t646;
    (t11890, t11893, t11896, t11897, t11898, t11901, t11902, t11909, t11922, t11923, t11928, t11929, t11930)
}
