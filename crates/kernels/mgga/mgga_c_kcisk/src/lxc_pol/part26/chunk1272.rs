//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1272/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1272<F: Float>(t53214: F, t9795: F, t9426: F, t33392: F, t3748: F, t32096: F, t33451: F, t394: F, t6343: F, t32173: F, t33384: F, t32176: F, t33469: F, t32019: F, t20160: F, t33399: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113581 = t53214 * t9795;
    let t113582 = t9426 * t113581;
    let t113584 = t3748 * t33392;
    let t113598 = 0.69444444444444444446e-2 * t32096 * t33451;
    let t113599 = t6343 * t394;
    let t113604 = 0.69444444444444444446e-2 * t33384 * t32173;
    let t113606 = 0.69444444444444444446e-2 * t33384 * t32176;
    let t113620 = 0.23148148148148148148e-2 * t32096 * t33469;
    let t113622 = 0.23148148148148148148e-2 * t32019 * t33469;
    let t113639 = t20160 * t33399;
    (t113581, t113582, t113584, t113598, t113599, t113604, t113606, t113620, t113622, t113639)
}
