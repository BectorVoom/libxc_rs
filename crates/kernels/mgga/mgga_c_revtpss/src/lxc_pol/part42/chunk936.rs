//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 936/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk936<F: Float>(t1312: F, t2199: F, t2201: F, t4248: F, t651: F, t7732: F, t7889: F, t8393: F, t8407: F, t8411: F, t8413: F, t3: F) -> (F, F, F) {
    let t8416 = 2.0 * t1312 * t8411 + 2.0 * t1312 * t8413 - 2.0 * t2199 * t4248 - 2.0 * t2199 * t7732 + 2.0 * t2201 * t4248 + 2.0 * t2201 * t7889 - 2.0 * t651 * t8393 - 2.0 * t651 * t8407;
    let t8417 = t3 * t8416;
    let t8421 = param_d * t8416;
    (t8416, t8417, t8421)
}
