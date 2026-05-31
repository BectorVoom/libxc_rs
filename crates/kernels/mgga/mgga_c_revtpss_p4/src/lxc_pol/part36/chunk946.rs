//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 946/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk946<F: Float>(t4279: F, t5911: F, t22604: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t22597: F, t22600: F, t22605: F, t22608: F, t22618: F, t5902: F, t5908: F, t5912: F, t97: F) -> F {
    let t22621 = t4279 * t5911;
    let t22624 = -t22604;
    let t22625 = t108 * t22624;
    let t22628 = -F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t97 * t22597 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t97 * t22600 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t22605 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t22608 * t109 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t5902 * t1510 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t5908 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t1507 * t5912 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t105 * t22618 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t105 * t22621 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t22625;
    t22628
}
