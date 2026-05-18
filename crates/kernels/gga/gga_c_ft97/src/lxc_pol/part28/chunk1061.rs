//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1061/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1061<F: Float>(t34384: F, t432: F, t446: F, t8411: F, t34379: F, t38921: F, t5617: F, t6469: F, t34415: F, t1564: F, t18: F, t32350: F, t3281: F) -> (F, F, F, F, F) {
    let t145636 = t446 * t8411 * t34384 * t432;
    let t145640 = t446 * t38921 * t34379 * t432;
    let t145644 = t446 * t8411 * t6469 * t5617;
    let t145648 = t446 * t8411 * t34415 * t432;
    let t145652 = t3281 * t1564 * t32350 * t18;
    (t145636, t145640, t145644, t145648, t145652)
}
