//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1105/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1105<F: Float>(t20624: F, t2243: F, t2250: F, t6682: F, t816: F, t2282: F, t2311: F, t6666: F, t835: F, t20730: F, t275: F, t2289: F, t6640: F, t20741: F, t2272: F, t6709: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20960 = 0.17757530864197530864e0 * t20624;
    let t20972 = t2243 * t2250;
    let t20975 = t816 * t6682;
    let t20990 = t2282 * t2311;
    let t20995 = t835 * t6666;
    let t21000 = t275 * t20730;
    let t21004 = t2282 * t2289;
    let t21007 = t835 * t6640;
    let t21037 = t275 * t20741;
    let t21043 = t2243 * t2272;
    let t21048 = t816 * t6709;
    (t20960, t20972, t20975, t20990, t20995, t21000, t21004, t21007, t21037, t21043, t21048)
}
