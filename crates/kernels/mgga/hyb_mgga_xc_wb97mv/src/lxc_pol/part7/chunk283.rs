//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 283/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk283<F: Float>(t941: F, t946: F, t238: F, t351: F, t800: F, t341: F, t929: F, t242: F, t931: F, t942: F, t944: F) -> (F, F, F, F, F, F) {
    let t947 = t946 * t941;
    let t950 = t238 * t800 * t351;
    let t951 = 0.82156666666666666667e-1 * t950;
    let t952 = t341 * t929;
    let t954 = t238 * t242 * t952;
    let t956 = 0.1898925e1 * t942 - t944 + 0.8969e0 * t931 + 0.3071625e0 * t947 - t951 + 0.24647e0 * t954;
    (t947, t950, t951, t952, t954, t956)
}
