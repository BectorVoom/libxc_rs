//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 867/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk867<F: Float>(t33994: F, t7839: F, t8779: F, t1089: F, t535: F, t7553: F, t7554: F, t7637: F, t8491: F, t1967: F, t8536: F, t30543: F, t8661: F, t30219: F, t8610: F, t30937: F, t8614: F) -> (F, F, F, F, F, F, F, F) {
    let t33995 = 0.7145669686344956162e-3 * t33994;
    let t33996 = t7839 * t8779;
    let t33997 = 0.42874018118069736972e-3 * t33996;
    let t34009 = t7553 * t1089 * t535 * t7554;
    let t34011 = t7637 * t8491;
    let t34013 = t1967 * t8536;
    let t34014 = 0.64311027177104605458e-2 * t34013;
    let t34023 = t30543 * t8661;
    let t34024 = 0.28303283060643736861e-1 * t34023;
    let t34027 = t30219 * t8610;
    let t34028 = 0.21437009059034868486e-2 * t34027;
    let t34029 = t30937 * t8614;
    (t33995, t33997, t34009, t34011, t34014, t34024, t34028, t34029)
}
