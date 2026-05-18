//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1222/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1222<F: Float>(t1032: F, t11007: F, t233: F, t25372: F, t1957: F, t2718: F, t25386: F, t7015: F, t9292: F, t1955: F, t7056: F, t10867: F, t867: F) -> (F, F, F, F, F, F, F) {
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    let t93314 = t25372 * t93301;
    let t93317 = t25386 * t93280;
    let t93334 = F::new(0.17073386770573548589e-1) * t9292 * t7015;
    let t93349 = t1955 * t7056 * t11007;
    let t93355 = t867 * t10867;
    (t93281, t93302, t93314, t93317, t93334, t93349, t93355)
}
