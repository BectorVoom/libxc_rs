//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1187/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1187<F: Float>(t17752: F, t2030: F, t9711: F, t4262: F, t513: F, t8539: F, t1524: F, t2297: F, t7447: F, t9712: F, t20555: F, t7450: F, t8915: F) -> (F, F, F, F, F) {
    let t40361 = t2030 * t17752 * t9711;
    let t40365 = t2030 * t4262 * t8539 * t513;
    let t40369 = t2030 * t4262 * t2297 * t1524;
    let t40371 = t7447 * t9712;
    let t40374 = t7450 * t20555 * t8915;
    (t40361, t40365, t40369, t40371, t40374)
}
