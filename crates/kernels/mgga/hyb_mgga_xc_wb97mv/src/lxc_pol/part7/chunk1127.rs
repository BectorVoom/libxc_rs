//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1127/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1127<F: Float>(t2003: F, t2037: F, t2035: F, t2040: F, t3167: F, t683: F, t688: F, t2045: F, t6715: F, t2032: F, t2064: F, t10: F, t138: F, t2065: F, t150: F, t168: F) -> (F, F, F, F, F, F, F, F) {
    let t21951 = t2003 * t2037;
    let t21953 = t2035 * t21951 * t2040;
    let t21962 = t683 * t3167 * t688;
    let t21976 = t683 * t6715 * t2045;
    let t21982 = 1.0 / t2064 / t2032;
    let t21983 = t21982 * t10;
    let t21986 = 1.0 / t138 / t2065;
    let t22001 = 1.0 / t168 / t150;
    (t21951, t21953, t21962, t21976, t21982, t21983, t21986, t22001)
}
