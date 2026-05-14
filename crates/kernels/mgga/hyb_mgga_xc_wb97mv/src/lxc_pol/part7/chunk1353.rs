//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1353/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1353<F: Float>(t11693: F, t9897: F, t3679: F, t3809: F, t3813: F, t3791: F, t3795: F, t1111: F, t4955: F, t11964: F, t1114: F, t14407: F, t7899: F, t28043: F, t10162: F, t10186: F, t10194: F, t28042: F, t28048: F, t28342: F, t28378: F, t28395: F, t28410: F, t28617: F, t28621: F, t28677: F, t28682: F, t28838: F, t32598: F, t9831: F, sigma0: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33139 = t11693 * t9897;
    let t33143 = t3809 * tau0 * t3679;
    let t33147 = t3813 * tau0 * t3679;
    let t33151 = t3791 * tau0 * t3679;
    let t33155 = t3795 * tau0 * t3679;
    let t33162 = t4955 * t1111;
    let t33163 = t11964 * t33162;
    let t33166 = t4955 * t1114;
    let t33167 = t11964 * t33166;
    let t33173 = t14407 * t7899;
    let t33176 = t4955 * sigma0;
    let t33177 = t28043 * t33176;
    let t33184 = 0.14222222222222222222e-2 * t10194 * t32598 - 0.37333333333333333333e1 * t10162 * t33139 + 0.8e0 * t28621 * t33143 - 0.16e1 * t28838 * t33147 - 8.0 * t28395 * t33151 + 12.0 * t28617 * t33155 + 0.168e2 * t28410 * t33143 - 0.224e2 * t28378 * t33147 - 0.192e-1 * t28048 * t33163 + 0.224e-1 * t28677 * t33167 + 0.4608e-4 * t28342 * t14407 * t9831 - 0.55296e-4 * t28682 * t33173 - 0.512e-4 * t10186 * t33177 - 0.16e-1 * t28042 * t33163 + 0.192e-1 * t28048 * t33167;
    (t33139, t33143, t33147, t33151, t33155, t33163, t33167, t33173, t33177, t33184)
}
