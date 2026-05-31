//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1491/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491<F: Float>(t1921: F, t8330: F, t1913: F, t8349: F, t31512: F, t571: F, t31463: F, t575: F, t1464: F, t8416: F, t1455: F, t8433: F) -> (F, F, F, F, F, F) {
    let t118091 = F::cast_from(2.0_f64) * t8330 * t1921;
    let t118094 = F::cast_from(2.0_f64) * t1913 * t8349;
    let t118099 = F::cast_from(2.0_f64) * t571 * t31512;
    let t118106 = F::cast_from(2.0_f64) * t31463 * t575;
    let t118108 = F::cast_from(2.0_f64) * t8416 * t1464;
    let t118110 = F::cast_from(2.0_f64) * t1455 * t8433;
    (t118091, t118094, t118099, t118106, t118108, t118110)
}
