//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 622/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk622<F: Float>(t1800: F, t25990: F, t1317: F, t28: F, t23048: F, t23055: F, t25958: F, t25962: F, t25966: F, t25970: F, t25973: F, t25976: F, t25979: F, t25983: F, t25988: F) -> (F, F) {
    let t25991 = t1800 * t25990;
    let t25993 = t1317 * t28 * t25991;
    let t25995 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t23048 + t25958 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t25962 - F::cast_from(2.0_f64) * t25966 - t23055 / F::cast_from(54.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t25970 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t25973 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t25976 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t25979 - t25983 / F::cast_from(36.0_f64) - t25988 / F::cast_from(36.0_f64) + t25993 / F::cast_from(3.0_f64);
    (t25993, t25995)
}
