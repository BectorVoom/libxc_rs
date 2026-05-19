//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 160/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk160<F: Float>(t571: F, t574: F, t577: F, t581: F, t591: F, t45: F, t589: F) -> (F, F, F, F, F) {
    let t596 = F::new(0.51785e1) * t574 + F::new(0.905775e0) * t571 + F::new(0.1100325e0) * t577 + F::new(0.1241775e0) * t581;
    let t599 = F::new(1.0) + F::cast_from(0.29608574643216675549e2_f64) / t596;
    let t600 = F::ln(t599);
    let t601 = t591 * t600;
    let t604 = -t589 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t601;
    (t596, t599, t600, t601, t604)
}
