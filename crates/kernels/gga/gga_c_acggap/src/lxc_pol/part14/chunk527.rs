//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 527/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk527<F: Float>(t1629: F, t4162: F, t1160: F, t441: F, t524: F, t1004: F, t1648: F, t1529: F, t310: F, t1633: F, t157: F, t864: F, t3088: F, t1642: F, t3378: F, t1539: F) -> (F, F, F, F, F, F, F, F) {
    let t4163 = t1629 * t4162;
    let t4164 = t1160 * t4163;
    let t4166 = t441 * t524;
    let t4170 = t1004 * t1648;
    let t4180 = t310 * t1529;
    let t4182 = 0.26341796731742046394e1 * t4180 * t1633;
    let t4183 = t157 * t864;
    let t4184 = t1629 * t4183;
    let t4185 = t3088 * t4184;
    let t4188 = 0.13170898365871023197e1 * t3378 * t1642;
    let t4189 = t4166 * t1539;
    (t4164, t4170, t4180, t4182, t4183, t4185, t4188, t4189)
}
