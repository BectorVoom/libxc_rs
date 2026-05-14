//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1164/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1164<F: Float>(t32343: F, t32390: F, t32439: F, t33350: F, t33353: F, t33364: F, t33762: F, t33767: F, t33771: F, t33778: F, t33784: F, t9512: F, t9519: F, t9536: F, t9855: F, t9535: F, t9850: F) -> (F, F) {
    let t33788 = -0.52083333333333333333e-2 * t9536 * t33762 - 0.5787037037037037037e-3 * t32343 + 0.20104166666666666667e-2 * t33767 * t9519 + 0.6701388888888888889e-3 * t32439 * t33771 + 0.77382407407407407407e-3 * t33350 - 0.11607361111111111111e-2 * t33353 + 0.52083333333333333333e-2 * t9512 * t9855 + 0.20104166666666666667e-2 * t33778 * t9519 - 0.60312500000000000001e-2 * t32439 * t33784 + 0.11607361111111111111e-2 * t33364 + t32390;
    let t33794 = t9850 * t9535;
    (t33788, t33794)
}
