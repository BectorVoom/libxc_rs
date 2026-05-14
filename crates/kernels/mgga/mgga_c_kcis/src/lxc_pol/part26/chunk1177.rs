//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1177/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1177<F: Float>(t22213: F, t303: F, t7931: F, t102278: F, t28747: F, t95024: F, t1610: F, t6281: F, t1615: F, t6159: F, t95103: F, t21854: F, t4160: F, t98266: F, t102081: F, t102280: F, t20984: F, t27567: F, t27583: F, t28758: F, t28765: F, t28807: F, t4440: F, t8222: F, t99219: F, t99301: F, t99556: F) -> (F, F, F, F, F, F, F) {
    let t102563 = t303 * t7931 * t22213;
    let t102568 = t95024 * t102278 * t28747;
    let t102575 = t6281 * t1610;
    let t102580 = t6281 * t1615;
    let t102582 = t6159 * t95103 * t102580;
    let t102586 = t4160 * t98266 * t21854;
    let t102594 = -0.17411041666666666666e-2 * t102563 + 0.61782407407407407408e-3 * t99219 * t8222 - 0.30918233506944444444e-4 * t27567 * t102568 - 0.92754700520833333333e-4 * t27567 * t102280 - 0.61836467013888888888e-4 * t27567 * t102081 + t99556 - 0.23168402777777777778e-3 * t27583 * t4440 * t28758 * t102575 - 0.23168402777777777778e-3 * t27583 * t102582 - 0.51588271604938271603e-3 * t102586 + 0.23168402777777777778e-3 * t99301 * t28807 - 0.69505208333333333334e-3 * t27583 * t6159 * t28765 * t20984;
    (t102563, t102568, t102575, t102580, t102582, t102586, t102594)
}
