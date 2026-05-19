//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 196/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk196<F: Float>(t571: F, t574: F, t577: F, t581: F, t240: F, t589: F, t601: F, t755: F) -> (F, F, F, F) {
    let t760 = F::new(0.705945e1) * t574 + F::new(0.1549425e1) * t571 + F::new(0.420775e0) * t577 + F::new(0.1562925e0) * t581;
    let t763 = F::new(1.0) + F::cast_from(0.32164683177870697974e2_f64) / t760;
    let t764 = F::ln(t763);
    let t772 = -t589 + t240 * (-F::new(0.3109e-1) * t755 * t764 + t589 - F::cast_from(0.19751789702565206229e-1_f64) * t601) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t601;
    (t760, t763, t764, t772)
}
