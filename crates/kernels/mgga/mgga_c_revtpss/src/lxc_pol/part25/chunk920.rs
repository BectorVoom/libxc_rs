//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 920/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk920<F: Float>(t283: F, t2857: F, t66: F, t11145: F, t247: F, t3298: F, t994: F, t4891: F, t3154: F, t999: F, t11659: F, t3117: F, t1086: F, t3046: F, t3090: F, t1043: F, t3075: F) -> (F, F, F, F, F, F, F) {
    let t11852 = 1.0 / t283 / t2857;
    let t11853 = t66 * t11852;
    let t11855 = t247 * t11853 * t11145;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11860 = t3154 * t999;
    let t11861 = t11659 * t11860;
    let t11862 = t3117 * t11861;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11869 = t3075 * t1043;
    (t11855, t11858, t11859, t11862, t11865, t11866, t11869)
}
