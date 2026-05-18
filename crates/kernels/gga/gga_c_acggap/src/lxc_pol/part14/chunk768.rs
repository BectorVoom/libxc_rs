//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 768/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk768<F: Float>(t1165: F, t1460: F, t8600: F, t7564: F, t1432: F, t604: F, t1181: F, t7426: F, t1439: F, t7575: F, t7351: F, t2016: F, t2282: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8602 = t1165 * t8600 * t1460;
    let t8603 = t7564 * t8602;
    let t8605 = t604 * t1432;
    let t8606 = t1181 * t8605;
    let t8607 = t7426 * t8606;
    let t8609 = t604 * t1439;
    let t8610 = t1181 * t8609;
    let t8611 = t7575 * t8610;
    let t8613 = t7351 * t1460;
    let t8614 = t1181 * t8613;
    let t8615 = t7564 * t8614;
    let t8619 = t2016 * t2282;
    (t8602, t8603, t8605, t8606, t8607, t8609, t8610, t8611, t8613, t8614, t8615, t8619)
}
