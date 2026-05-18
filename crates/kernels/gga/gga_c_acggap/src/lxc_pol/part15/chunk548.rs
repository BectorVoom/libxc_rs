//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 548/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk548<F: Float>(t1160: F, t4163: F, t441: F, t524: F, t1004: F, t1648: F, t1529: F, t310: F, t1633: F, t157: F, t864: F, t1629: F) -> (F, F, F, F, F, F, F) {
    let t4164 = t1160 * t4163;
    let t4166 = t441 * t524;
    let t4170 = t1004 * t1648;
    let t4180 = t310 * t1529;
    let t4182 = F::new(0.26341796731742046394e1) * t4180 * t1633;
    let t4183 = t157 * t864;
    let t4184 = t1629 * t4183;
    (t4164, t4166, t4170, t4180, t4182, t4183, t4184)
}
