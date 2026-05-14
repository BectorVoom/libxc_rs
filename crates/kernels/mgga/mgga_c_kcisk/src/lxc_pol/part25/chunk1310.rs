//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1310/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1310<F: Float>(t32903: F, t34103: F, t6713: F, t3805: F, t9957: F, t112451: F, t112518: F, t112521: F, t112523: F, t116276: F, t116285: F, t116384: F, t15921: F, t32893: F, t32942: F, t32948: F, t33002: F, t33031: F, t34037: F, t34078: F, t34182: F, t34192: F, t695: F, t7242: F, t7246: F, t9649: F, t9664: F, t9665: F) -> (F, F, F) {
    let t116825 = t6713 * t32903 * t34103;
    let t116836 = t3805 * t9957;
    let t116850 = 0.40208333333333333335e-2 * t34192 * t32893 - 0.46296296296296296298e-2 * t112518 - 0.69444444444444444446e-2 * t112521 - 0.66327777777777777776e-2 * t116825 - 0.73697530864197530861e-3 * t112523 + 0.69444444444444444446e-2 * t9664 * t7246 * t9665 * t695 + 0.69444444444444444446e-2 * t33031 * t7242 * t34037 * t15921 + 0.14739506172839506172e-2 * t116836 - 0.20833333333333333334e-1 * t32942 * t34182 - 0.40208333333333333335e-2 * t9649 * t116384 - 0.24125000000000000001e-1 * t32948 * t34078 - 0.46561250000000000002e-2 * t33002 * t116285 + 0.13968375e-1 * t33002 * t116276 - 0.46561250000000000002e-2 * t112451 * t34078;
    (t116825, t116836, t116850)
}
