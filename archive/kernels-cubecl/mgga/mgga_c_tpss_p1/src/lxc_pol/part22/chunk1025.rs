//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1025/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1025<F: Float>(t10980: F, t10986: F, t11003: F, t11005: F, t11006: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8687: F) -> F {
    let t11040 = -t8687 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8616 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8607 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8618 + t8605 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10980 + t11003 - t11005 + t11006 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t11010 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11015 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11020 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11024 - F::cast_from(2.0_f64) * t11028 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11033 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11037 - t10986 / F::cast_from(3.0_f64);
    t11040
}
