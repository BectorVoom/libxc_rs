//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 932/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk932<F: Float>(t1998: F, t4625: F, t2001: F, t5113: F, t5118: F, t1434: F, t7736: F, t1418: F, t7614: F, t1089: F, t598: F, t6337: F, t7679: F, t1165: F, t19834: F, t2068: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t34745 = t1998 * t4625;
    let t34747 = t2001 * t5113;
    let t34749 = t2001 * t5118;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34757 = t598 * t1089 * t6337 * t7679;
    let t34762 = t2068 * t1165 * t604 * t19834;
    (t34745, t34747, t34749, t34751, t34753, t34757, t34762)
}
