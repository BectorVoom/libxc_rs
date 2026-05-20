//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1649/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649<F: Float>(t41549: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t41296: F, t87145: F) -> (F, F) {
    let t88100 = F::cast_from(0.47488888888888888888e-1_f64) * t77559 - F::cast_from(0.14246666666666666667e0_f64) * t77561 + F::cast_from(0.26382716049382716049e-1_f64) * t77499 - F::cast_from(0.31659259259259259258e-1_f64) * t63453 + F::cast_from(0.94977777777777777776e-1_f64) * t63459 + t41549 + F::new(0.4274e0) * t88085 - F::new(0.6411e0) * t88089 + F::new(0.10685e0) * t88093 + F::cast_from(0.14246666666666666667e0_f64) * t88097 - F::cast_from(0.47488888888888888888e-1_f64) * t63464;
    let t88102 = t41296 * t87145;
    (t88100, t88102)
}
