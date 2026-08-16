//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 623/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk623<F: Float>(t24544: F, t24642: F, t27799: F, t27803: F, t27808: F, t27811: F, t27817: F, t27823: F, t27826: F, t27830: F, t27834: F, t27839: F) -> F {
    let t28082 = -t27799 / F::cast_from(6.0_f64) + t27803 / F::cast_from(18.0_f64) - t27808 / F::cast_from(9.0_f64) - t24642 + t27811 / F::cast_from(9.0_f64) - t24544 / F::cast_from(54.0_f64) - t27817 / F::cast_from(6.0_f64) - t27823 / F::cast_from(8.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t27826 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27830 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27834 + t27839 / F::cast_from(3.0_f64);
    t28082
}
