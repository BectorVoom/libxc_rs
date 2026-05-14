//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1302/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1302<F: Float>(t11317: F, t238: F, t800: F, t242: F, t2462: F, t4283: F, t11280: F, t929: F, t31808: F, t341: F, t23174: F, t23180: F, t23281: F, t31771: F, t31812: F, t31817: F, t31820: F) -> (F, F, F, F, F) {
    let t31823 = t238 * t800 * t11317;
    let t31827 = t238 * t242 * t2462 * t4283;
    let t31831 = t238 * t242 * t929 * t11280;
    let t31835 = t238 * t242 * t341 * t31808;
    let t31838 = -0.65725333333333333333e0 * t31771 + 0.1898925e1 * t31812 - 0.1460562962962962963e1 * t23174 + 0.49294e0 * t31817 - 0.32862666666666666666e0 * t31820 - 0.32862666666666666666e0 * t31823 + 0.24647e0 * t31827 + 0.49294e0 * t31831 + 0.24647e0 * t31835 + t23281 - 0.18602370370370370371e1 * t23180;
    (t31823, t31827, t31831, t31835, t31838)
}
