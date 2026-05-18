//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 721/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk721<F: Float>(t390: F, t7736: F, t1998: F, t993: F, t1035: F, t1997: F) -> (F, F, F) {
    let t7737 = t7736 * t390;
    let t7738 = F::new(0.85748036236139473944e-3) * t7737;
    let t7739 = t1998 * t993;
    let t7740 = F::new(0.42874018118069736972e-3) * t7739;
    let t7741 = t1035 * t1997;
    (t7738, t7740, t7741)
}
