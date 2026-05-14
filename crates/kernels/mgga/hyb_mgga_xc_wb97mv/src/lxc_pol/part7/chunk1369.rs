//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1369/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1369<F: Float>(t4951: F, t28043: F, t1117: F, t1128: F, t1158: F, t1161: F, t11659: F, t28053: F, t2839: F, t2869: F, t2873: F, t28755: F, t2880: F, t2890: F, t2953: F, t2957: F, t33163: F, t33167: F, t33685: F, t33689: F, t33692: F, t33698: F, t33783: F, t4533: F, t4550: F, t4554: F, t4588: F, t4636: F, t4639: F, t511: F, t7908: F, t7927: F, t8025: F, t8034: F, t8081: F, sigma0: F) -> (F, F) {
    let t33842 = t4951 * sigma0;
    let t33843 = t28043 * t33842;
    let t33846 = 0.36e0 * t7927 * t1128 * t4550 * t2839 + 0.756e0 * t8081 * t33698 + 0.176e0 * t1158 * t33685 - 0.54e0 * t8025 * t1128 * t4554 * t2839 - 0.1008e1 * t7908 * t33689 + 0.88e0 * t2953 * t4533 * t8034 + 0.1232e1 * t2957 * t33783 - 0.176e0 * t1161 * t33692 + 12.0 * t1117 * t4588 * t2869 - 24.0 * t511 * t11659 * t2873 - t2890 * t4639 - 4.0 * t4636 * t2880 - 0.21333333333333333334e-2 * t28053 * t33163 + 0.32e-2 * t28755 * t33167 + 0.21333333333333333333e-2 * t28053 * t33843;
    (t33843, t33846)
}
