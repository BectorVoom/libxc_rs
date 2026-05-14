//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 847/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk847<F: Float>(t1133: F, t6613: F, t1131: F, t1096: F, t1092: F, t3211: F, t6276: F, t3210: F, t3200: F, t19552: F, t9512: F, t4554: F, t19679: F, t4580: F, t14381: F, t1014: F, t6483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19735 = t6613 * t1133;
    let t19736 = t1131 * t19735;
    let t19737 = t1096 * t19736;
    let t19738 = t1092 * t19737;
    let t19741 = t3211 * t6276 * t1133;
    let t19742 = t3210 * t19741;
    let t19743 = t3200 * t19742;
    let t19745 = t9512 * t19552;
    let t19746 = t3210 * t19745;
    let t19747 = t4554 * t19746;
    let t19750 = t4580 * t19679;
    let t19751 = t14381 * t19750;
    let t19752 = t3200 * t19751;
    let t19754 = t1014 * t6483;
    (t19735, t19738, t19741, t19743, t19745, t19747, t19750, t19752, t19754)
}
