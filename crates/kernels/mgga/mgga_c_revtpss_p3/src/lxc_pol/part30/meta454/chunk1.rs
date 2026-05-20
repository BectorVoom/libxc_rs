//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1729/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1729<F: Float>(t1269: F, t1287: F, t5284: F, t17633: F, t5458: F, t17482: F, t3769: F, t3783: F, t12713: F, t5332: F, t13147: F, t487: F) -> (F, F, F, F, F, F) {
    let t17826 = t1269 * t5284 * t1287;
    let t17829 = t17633 * t5458;
    let t17834 = t17482 * t3769;
    let t17837 = t17482 * t3783;
    let t17840 = t5332 * t12713;
    let t17845 = t13147 * t487;
    (t17826, t17829, t17834, t17837, t17840, t17845)
}
