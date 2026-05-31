//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1999/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1999<F: Float>(t29551: F, t7349: F, t101870: F, t101872: F, t101874: F, t101879: F, t101881: F, t108749: F, t108759: F, t109976: F, t109980: F, t109983: F, t109985: F, t109988: F, t6960: F, t7343: F) -> F {
    let t109990 = t29551 * t7349;
    let t109992 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t108749 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t109976 * t6960 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t109980 * t108759 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t109983 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t109985 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t109988 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t109990 + t101870 + t101872 + t101874 + t101879 + t101881;
    t109992
}
