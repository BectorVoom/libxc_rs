//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2249/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2249<F: Float>(t27833: F, t7935: F, t1448: F, t6922: F, t28196: F, t28197: F, t28067: F, t98450: F, t7897: F, t8995: F, t28199: F, t25082: F, t27153: F, t33651: F) -> (F, F, F, F, F) {
    let t109262 = F::cast_from(2.0_f64) * t27833 * t7935;
    let t109263 = t6922 * t1448;
    let t109266 = F::cast_from(2.0_f64) * t28196 * t28197 * t109263;
    let t109268 = F::cast_from(6.0_f64) * t98450 * t28067;
    let t109269 = t7897 * t8995;
    let t109271 = F::cast_from(4.0_f64) * t109269 * t28199;
    let t109274 = F::cast_from(6.0_f64) * t25082 * t33651 * t27153;
    (t109262, t109266, t109268, t109271, t109274)
}
