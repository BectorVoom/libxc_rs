//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2223/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223<F: Float>(t1469: F, t1925: F, t603: F, t4186: F, t77: F, t84: F, t2242: F, t5826: F, t19680: F, t108733: F, t108737: F, t108745: F, t108749: F, t108753: F, t1928: F, t25099: F, t25106: F, t29544: F, t29548: F, t6958: F, t6960: F) -> F {
    let t108757 = t603 * t1469 * t1925;
    let t108759 = t77 * t84 * t4186;
    let t108762 = t2242 * t5826;
    let t108765 = t603 * t19680;
    let t108768 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t25099 * t29544 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t25106 * t29544 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6958 * t108733 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6958 * t108737 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t25099 * t29548 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t25106 * t29548 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t108745 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t108749 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t108753 * t6960 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108757 * t108759 + t108762 * t1928 / F::cast_from(3.0_f64) + t108765 * t1928 / F::cast_from(3.0_f64);
    t108768
}
