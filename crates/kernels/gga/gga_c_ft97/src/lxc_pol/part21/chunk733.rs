//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 733/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk733<F: Float>(t16976: F, t17038: F, t17098: F, t17144: F, t17194: F, t17373: F, t17425: F, t17504: F, t160: F, t17486: F, t21: F, t3658: F, t1079: F, t4431: F, t649: F, t184: F, t920: F) -> (F, F, F, F, F) {
    let t17507 = t16976 + t17038 + t17098 + t17144 + t17194 + t17373 + t17425 + t17504;
    let t17510 = t17486 * t160;
    let t17531 = t21 * t3658;
    let t17532 = t1079 * t17531;
    let t17535 = t649 * t4431;
    let t17538 = t184 * t920;
    (t17507, t17510, t17532, t17535, t17538)
}
