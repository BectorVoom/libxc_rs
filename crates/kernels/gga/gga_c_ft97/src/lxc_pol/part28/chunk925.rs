//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 925/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk925<F: Float>(t446: F, t5617: F, t6469: F, t8411: F, t34415: F, t432: F, t1564: F, t18: F, t32350: F, t3281: F, t137231: F, t920: F, t32325: F, t942: F, t1317: F, t1800: F, t28: F) -> (F, F, F, F, F, F) {
    let t145644 = t446 * t8411 * t6469 * t5617;
    let t145648 = t446 * t8411 * t34415 * t432;
    let t145652 = t3281 * t1564 * t32350 * t18;
    let t145656 = t446 * t1564 * t137231 * t920;
    let t145658 = t32325 * t942;
    let t145661 = t1317 * t28 * t1800 * t145658;
    (t145644, t145648, t145652, t145656, t145658, t145661)
}
