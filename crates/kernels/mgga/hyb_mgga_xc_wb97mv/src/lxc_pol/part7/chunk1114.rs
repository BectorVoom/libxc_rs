//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1114/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1114<F: Float>(t11928: F, t11930: F, t1522: F, t646: F, t3732: F, t11857: F, t10079: F, t10084: F, t10133: F, t11782: F, t11786: F, t11791: F, t11801: F, t11804: F, t11886: F, t11890: F, t11893: F, t11897: F, t11898: F, t11902: F, t11909: F, t11923: F, t2828: F, t2832: F, t3685: F, t3741: F, t3760: F, t7818: F, t7832: F, t7838: F, t7897: F, t7903: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t11931 = t11928 * t11930;
    let t11934 = t1522 * sigma0;
    let t11935 = t11934 * t646;
    let t11936 = t3732 * t11935;
    let t11939 = t3732 * t11857;
    let t11942 = 0.576e0 * t10079 * t11886 + 0.1008e-2 * t7818 * t11890 + 0.576e0 * t10079 * t11893 - 0.672e0 * t11897 * t11898 + 0.46933333333333333333e-3 * t3741 * t11902 + 0.2304e-5 * t2828 * t11782 - 0.2304e-5 * t2832 * t11786 - 0.3072e-5 * t11909 * t11791 + 0.72e-1 * t10133 * t11801 + 0.64e-1 * t7832 * t11804 + 0.64e-1 * t10084 * t11886 + 0.144e-3 * t7838 * t11890 + 0.64e-1 * t10084 * t11893 - 0.96e-1 * t11923 * t11898 + 0.46933333333333333333e-3 * t3760 * t11902 + 0.9216e-8 * t3685 * t11931 - 0.1728e-2 * t7903 * t11936 - 0.192e-3 * t7897 * t11939;
    (t11931, t11934, t11935, t11936, t11939, t11942)
}
