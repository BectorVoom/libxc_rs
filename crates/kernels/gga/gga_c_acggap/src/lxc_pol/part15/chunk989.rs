//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 989/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk989<F: Float>(t30777: F, t2290: F, t7630: F, t1549: F, t30540: F, t1554: F, t1558: F, t30137: F, t7585: F, t8525: F, t1072: F, t535: F, t7507: F, t7512: F) -> (F, F, F, F, F, F, F) {
    let t34848 = F::cast_from(0.17149607247227894789e-2_f64) * t30777;
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34865 = t7585 * t30137 * t8525;
    let t34879 = t7507 * t7512 * t535 * t1072;
    (t34848, t34849, t34851, t34853, t34855, t34865, t34879)
}
