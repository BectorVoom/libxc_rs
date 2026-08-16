//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2215/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215<F: Float>(t2121: F, t2247: F, t5819: F, t1469: F, t603: F, t108737: F, t108745: F, t108749: F, t108759: F, t108762: F, t108765: F, t108816: F, t2123: F, t26749: F, t26755: F, t29375: F, t29548: F, t29554: F, t6960: F, t7566: F, t7576: F, t7709: F) -> F {
    let t111453 = t2247 * t5819 * t2121;
    let t111457 = t603 * t1469 * t2121;
    let t111468 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t29375 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7566 * t108737 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t26749 * t29548 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t26755 * t29548 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t108745 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t108749 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t111453 * t6960 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t111457 * t108759 + t108762 * t2123 / F::cast_from(3.0_f64) + t108765 * t2123 / F::cast_from(3.0_f64) + t108816 * t2123 / F::cast_from(3.0_f64) + t29554 * t7576 / F::cast_from(3.0_f64);
    t111468
}
