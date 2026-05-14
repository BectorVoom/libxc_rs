//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1114/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1114<F: Float>(t34298: F, t98588: F, t2014: F, t28926: F, t8717: F, t28182: F, t8698: F, t34261: F, t7374: F, t32392: F, t7978: F, t32394: F, t28760: F, t8634: F, t34167: F, t649: F) -> (F, F, F, F, F, F, F, F) {
    let t128869 = 2.0 * t98588 * t34298;
    let t128871 = t2014 * t28926 * t8717;
    let t128874 = t8698 * t28182;
    let t128876 = 2.0 * t34261 * t7374;
    let t128878 = 2.0 * t32392 * t7978;
    let t128880 = 2.0 * t32394 * t7978;
    let t128882 = 2.0 * t8634 * t28760;
    let t128891 = t649 * t34167;
    (t128869, t128871, t128874, t128876, t128878, t128880, t128882, t128891)
}
