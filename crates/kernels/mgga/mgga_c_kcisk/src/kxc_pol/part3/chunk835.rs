//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 835/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk835<F: Float>(t1234: F, t1264: F, t13675: F, t13680: F, t13683: F, t13697: F, t13702: F, t13705: F, t13708: F, t13711: F, t13717: F, t13752: F, t374: F, t4031: F, t4033: F, t4081: F, t4096: F, t4122: F, t4130: F, t45: F) -> (F,) {
    let t13759 = 1.0 * t1234 * t13675 + 0.51725014705706168417e3 * t13680 * t13683 + 0.19751789702565206229e-1 * t45 * t13697 * t374 + 0.48245472966453314466e2 * t4081 * t13702 - 6.0 * t13705 * t4033 + 6.0 * t4081 * t13708 - 6.0 * t4031 * t13711 + 0.1038945353962551798e3 * t1264 * t13717 - 0.58482233974552040708e0 * t1264 * t13752 - 0.17544670192365612213e1 * t4096 * t4122 - 0.51947267698127589899e2 * t4096 * t4130;
    (t13759,)
}
