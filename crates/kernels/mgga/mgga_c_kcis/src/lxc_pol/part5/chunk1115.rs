//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1115/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1115<F: Float>(t174: F, t2641: F, t6284: F, t176: F, t18431: F, t20840: F, t2633: F, t4518: F, t833: F, t20839: F, t44: F, t230: F, t18435: F, t18438: F, t18441: F, t18447: F, t18449: F, t18451: F, t18454: F, t18456: F, t20819: F, t20821: F, t20824: F, t20826: F, t8524: F, t9272: F, t9313: F, t9315: F, zeta_threshold: F) -> (F,) {
    let t175 = t174 <= zeta_threshold;
    let t20845 = t2641 * t6284;
    let t20851 = piecewise3(t175, 0.0, -8.0 / 27.0 * t20840 * t833 - 16.0 / 9.0 * t4518 * t2633 + 4.0 / 9.0 * t20845 * t833 + 4.0 / 3.0 * t176 * t18431);
    let t20853 = (t20839 + t20851) * t44;
    let t20854 = t20853 * t230;
    let t20855 = -t18435 / 16.0 + t8524 + t9315 - t18438 / 8.0 - t18441 / 8.0 - t18447 / 16.0 - t9313 - t18449 / 8.0 + t18451 / 16.0 - t18454 / 16.0 + t18456 / 8.0 - t20819 / 16.0 - t9272 + t20821 / 8.0 - t20824 / 8.0 + t20826 / 8.0 + t20854;
    (t20855,)
}
