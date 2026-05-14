//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 195/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk195<F: Float>(t571: F, t574: F, t577: F, t581: F, t240: F, t589: F, t601: F, t755: F) -> (F, F, F, F) {
    let t760 = 0.705945e1 * t574 + 0.1549425e1 * t571 + 0.420775e0 * t577 + 0.1562925e0 * t581;
    let t763 = 1.0 + 0.32164683177870697974e2 / t760;
    let t764 = f64::ln(t763);
    let t772 = -t589 + t240 * (-0.3109e-1 * t755 * t764 + t589 - 0.19751789702565206229e-1 * t601) + 0.19751789702565206229e-1 * t240 * t601;
    (t760, t763, t764, t772)
}
