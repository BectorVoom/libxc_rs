//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1232/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1232<F: Float>(t1620: F, t9891: F, t9882: F, t1591: F, t2331: F, t32440: F, t6204: F) -> (F, F, F, F) {
    let t33750 = t9891 * t1620;
    let t33757 = t9882 * t1620;
    let t33760 = t2331 * t1591;
    let t33761 = t32440 * t33760;
    let t33762 = t6204 * t33761;
    (t33750, t33757, t33761, t33762)
}
