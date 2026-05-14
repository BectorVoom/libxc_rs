//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1205/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1205<F: Float>(t2922: F, t7654: F, t774: F, t7659: F, t7664: F, t7667: F, t54: F, t7699: F, t2899: F, t7769: F, t7702: F, t2096: F, t7692: F, t17848: F, t2104: F, t7641: F) -> (F, F, F, F, F, F, F, F) {
    let t21752 = t2922 * t774 * t7654;
    let t21755 = t2922 * t774 * t7659;
    let t21758 = t7664 * t774 * t7667;
    let t21787 = t54 * t7699;
    let t21789 = t2899 * t21787 * t7769;
    let t21794 = t2922 * t21787 * t7702;
    let t21841 = t2096 * t7692;
    let t21852 = t2104 * t17848 * t7641;
    (t21752, t21755, t21758, t21787, t21789, t21794, t21841, t21852)
}
