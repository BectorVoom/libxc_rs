//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1949/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1949<F: Float>(t28112: F, t7349: F, t28116: F, t28119: F, t26169: F, t7709: F, t60221: F, t7342: F, t6960: F, t95268: F, t95270: F, t95284: F, t95286: F, t95288: F, t95290: F, t95294: F) -> F {
    let t101879 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t28112 * t7349;
    let t101881 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t28116 * t7349;
    let t101883 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t28119 * t7349;
    let t101885 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7709 * t26169;
    let t101886 = t60221 * t7342;
    let t101896 = t101879 + t101881 + t101883 + t101885 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101886 * t6960 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t95268 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t95270 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t95284 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t95286 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t95288 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t95290 - F::cast_from(880.0_f64) / F::cast_from(27.0_f64) * t95294;
    t101896
}
