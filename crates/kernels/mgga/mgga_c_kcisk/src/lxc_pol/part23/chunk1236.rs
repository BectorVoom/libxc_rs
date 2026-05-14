//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1236/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1236<F: Float>(t9528: F, t9850: F, t9532: F, t9851: F, t21434: F, t79: F, t2736: F, t2740: F, t32339: F, t32354: F, t32391: F, t32399: F, t32402: F, t33368: F, t33794: F, t9524: F, t9539: F, t9544: F, t9855: F, t9860: F, t9864: F) -> (F, F, F, F) {
    let t33802 = t9850 * t9528;
    let t33805 = t9851 * t9532;
    let t33807 = t21434 * t79;
    let t33808 = t33807 * t2736;
    let t33813 = 0.23214722222222222222e-2 * t33368 + 0.46296296296296296296e-2 * t32391 - 0.17361111111111111111e-2 * t32399 + 0.46296296296296296296e-2 * t32339 * t9864 - 0.17361111111111111111e-2 * t33794 * t9539 - 0.17361111111111111111e-2 * t32354 * t9864 + 0.67013888888888888888e-3 * t32402 + 0.52083333333333333333e-2 * t9524 * t9855 + 0.13888888888888888889e-1 * t33802 * t2740 - 0.17361111111111111111e-2 * t33805 - 0.52083333333333333333e-2 * t33808 * t2740 + 0.52083333333333333333e-2 * t9860 * t9544;
    (t33802, t33807, t33808, t33813)
}
