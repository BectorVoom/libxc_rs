//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1092/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1092<F: Float>(t12248: F, t12251: F, t18326: F, t18423: F, t2013: F, t24993: F, t24998: F, t25002: F, t25007: F, t25011: F, t7591: F, t7606: F, t7611: F, t7615: F, t2630: F, t7624: F) -> (F, F) {
    let t25020 = 0.35981577432354634426e-1 * t18326 * t24993 + 0.35981577432354634426e-1 * t18326 * t24998 - 0.23987718288236422951e-1 * t18326 * t25002 + t12248 - 0.59969295720591057378e-2 * t12251 + t18423 - 0.59969295720591057377e-2 * t25007 - 0.89953943580886586067e-2 * t2013 * t25011 + 0.47975436576472845902e-1 * t7591 * t7611 + 0.95950873152945691804e-1 * t7591 * t7615 - 0.63967248768630461203e-1 * t7591 * t7606;
    let t25024 = t2630 * t7624;
    (t25020, t25024)
}
