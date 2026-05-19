//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 575/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk575<F: Float>(t1160: F, t4150: F, t1629: F, t930: F, t3084: F, t157: F, t879: F, t441: F, t524: F, t1533: F, t1004: F, t1648: F) -> (F, F, F, F, F, F, F, F) {
    let t4152 = F::cast_from(0.13170898365871023197e1_f64) * t1160 * t4150;
    let t4153 = t1629 * t930;
    let t4159 = t1629 * t3084;
    let t4162 = t157 * t879;
    let t4163 = t1629 * t4162;
    let t4164 = t1160 * t4163;
    let t4166 = t441 * t524;
    let t4167 = t4166 * t1533;
    let t4170 = t1004 * t1648;
    (t4152, t4153, t4159, t4162, t4164, t4166, t4167, t4170)
}
