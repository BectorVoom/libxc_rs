//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 732/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk732<F: Float>(t32106: F, t469: F, t28: F, t5665: F, t32061: F, t32066: F, t32072: F, t32080: F, t32085: F, t32089: F, t32093: F, t32097: F, t32101: F, t32104: F) -> (F, F, F) {
    let t32107 = t469 * t32106;
    let t32109 = t5665 * t28 * t32107;
    let t32111 = t32061 / F::cast_from(2.0_f64) + t32066 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t32072 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t32080 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t32085 - t32089 / F::cast_from(6.0_f64) - t32093 - t32097 / F::cast_from(9.0_f64) - t32101 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t32104 + t32109 / F::cast_from(12.0_f64);
    (t32107, t32109, t32111)
}
