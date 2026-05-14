//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1019/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1019<F: Float>(t1142: F, t518: F, t1126: F, t10070: F, t10095: F, t10111: F, t10117: F, t10121: F, t10133: F, t10141: F, t10143: F, t10147: F, t10150: F, t10152: F, t10156: F, t10162: F, t10166: F, t10172: F, t10173: F, t10177: F, t10178: F, t10182: F, t10186: F, t10190: F, t2817: F, t2823: F, t2832: F, t7832: F, t9853: F) -> (F, F, F) {
    let t10193 = t518 * t1142;
    let t10194 = t1126 * t10193;
    let t10197 = -0.144e-3 * t2832 * t10111 + 0.64e-1 * t7832 * t10095 + 0.72e-1 * t10133 * t10070 + 0.768e-6 * t2817 * t10117 - 0.768e-6 * t2823 * t10121 + 0.2e0 * t10141 * t10143 - 0.16e-2 * t10147 * t9853 - 1.0 * t10150 * t10152 + 0.12e1 * t10156 * t10143 - 0.12e1 * t10156 * t10152 + 0.14e1 * t10162 * t10143 + 0.13333333333333333333e0 * t10166 * t10143 - 0.13333333333333333333e0 * t10166 * t10152 + 0.1536e-5 * t10172 * t10173 - 0.1536e-5 * t10177 * t10178 - 0.16e-2 * t10182 * t9853 + 0.4608e-5 * t10186 * t10173 - 0.4608e-5 * t10190 * t10178 - 0.53333333333333333333e-3 * t10194 * t9853;
    (t10193, t10194, t10197)
}
