//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2002/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002<F: Float>(t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t3223: F, t7131: F, t11273: F, t25504: F, t25508: F, t11263: F, t7122: F) -> (F, F, F, F, F, F, F) {
    let t93750 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93764 = t3223 * t7131;
    let t93783 = t11273 * t25504;
    let t93796 = t11273 * t25508;
    let t93801 = t7122 * t11263;
    (t93750, t93751, t93752, t93764, t93783, t93796, t93801)
}
