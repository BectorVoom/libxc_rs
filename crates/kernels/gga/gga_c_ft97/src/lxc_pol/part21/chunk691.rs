//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 691/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk691<F: Float>(t16726: F, t446: F, t15746: F, t2205: F, t3281: F, t3408: F, t925: F, t1969: F, t4668: F, t7368: F, t558: F, t28: F, t89: F, t3342: F, t1546: F, t4664: F) -> (F, F, F, F, F, F, F) {
    let t16727 = t446 * t16726;
    let t16729 = t2205 * t15746;
    let t16730 = t3281 * t16729;
    let t16732 = t925 * t3408;
    let t16733 = t1969 * t16732;
    let t16734 = t446 * t16733;
    let t16736 = t7368 * t4668;
    let t16737 = t16736 * t558;
    let t16739 = t89 * t28 * t16737;
    let t16740 = t3342 * t3408;
    let t16742 = t89 * t28 * t16740;
    let t16745 = t89 * t1546 * t4664;
    (t16727, t16730, t16732, t16734, t16739, t16742, t16745)
}
