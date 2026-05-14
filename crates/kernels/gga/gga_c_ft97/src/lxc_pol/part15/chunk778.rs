//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 778/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk778<F: Float>(t38062: F, t12168: F, t70: F, t170: F, t180: F, t178: F, t2280: F, t159: F, t9437: F, t157: F, t10: F, t11175: F, t144: F, t1642: F, t1984: F, t525: F, t7954: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39546 = 0.14978012345679012345e1 * t38062;
    let t39600 = t12168 * t70;
    let t39603 = 220.0 / 81.0 * t170 * t39600 * t180;
    let t39616 = 1.0 / t2280 / t178;
    let t39652 = 1.0 / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39673 = t10 * t11175 * t144;
    let t39674 = 280.0 / 81.0 * t39673;
    let t39693 = t1642 * t1984;
    let t39725 = t7954 * t525;
    (t39546, t39600, t39603, t39616, t39653, t39673, t39674, t39693, t39725)
}
