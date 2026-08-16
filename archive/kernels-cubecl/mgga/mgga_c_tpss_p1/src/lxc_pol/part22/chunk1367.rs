//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1367/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1367<F: Float>(t65639: F, t65643: F, t65647: F, t60725: F, t60731: F, t60733: F, t60739: F, t60744: F, t60750: F, t60752: F, t65636: F, t65641: F, t65645: F) -> F {
    let t67183 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t65639;
    let t67185 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t65643;
    let t67187 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t65647;
    let t67191 = -F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t60725 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t60731 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t60733 + t65636 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t60739 - t67183 + t65641 / F::cast_from(192.0_f64) + t67185 - t65645 / F::cast_from(192.0_f64) - t67187 - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t60744 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t60750 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t60752;
    t67191
}
