//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2041/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2041<F: Float>(t11782: F, t1972: F, t1007: F, t25532: F, t3080: F, t7106: F, t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t11923: F, t25580: F) -> (F, F, F, F, F, F, F) {
    let t93736 = t11782 * t1972;
    let t93743 = t25532 * t1007;
    let t93745 = t7106 * t3080;
    let t93750 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93755 = t25580 * t11923;
    (t93736, t93743, t93745, t93750, t93751, t93752, t93755)
}
