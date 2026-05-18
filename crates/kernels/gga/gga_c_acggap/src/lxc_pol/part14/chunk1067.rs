//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1067/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1067<F: Float>(t1967: F, t9565: F, t1410: F, t525: F, t1181: F, t2068: F, t599: F, t38647: F, t157: F, t1782: F, t406: F, t7351: F) -> (F, F, F, F, F, F) {
    let t38820 = t1967 * t9565;
    let t38827 = t525 * t1410;
    let t38830 = t2068 * t1181 * t599 * t38827;
    let t38834 = t2068 * t1181 * t599 * t38647;
    let t38837 = t1782 * t406 * t157;
    let t38840 = t2068 * t1181 * t7351 * t38837;
    (t38820, t38827, t38830, t38834, t38837, t38840)
}
