//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 902/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk902<F: Float>(t242: F, t27989: F, t684: F, t6947: F, t724: F, t3859: F, t6154: F, t729: F, t1882: F, t6932: F, t6930: F, t14175: F, t1449: F, t2347: F, t3886: F, t14187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28326 = t242 * t27989;
    let t28330 = t724 * t6947 * t684;
    let t28334 = t729 * t6154 * t3859;
    let t28338 = t1882 * t6932;
    let t28340 = t6930 * t684;
    let t28341 = t14175 * t28340;
    let t28344 = t1449 * t2347;
    let t28345 = t28344 * t3886;
    let t28346 = t14187 * t28345;
    (t28326, t28330, t28334, t28338, t28340, t28341, t28344, t28345, t28346)
}
