//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 893/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk893<F: Float>(t33106: F, t33110: F, t33114: F, t34851: F, t34856: F, t34921: F, t34925: F, t34929: F, t34933: F, t34937: F, t34941: F, t34945: F) -> F {
    let t35148 = t33106 + t34851 / F::cast_from(18.0_f64) + t34856 / F::cast_from(3.0_f64) - t34921 / F::cast_from(6.0_f64) - t33110 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t34925 - F::cast_from(2.0_f64) * t34929 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t34933 + t33114 + t34937 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34941 - t34945 / F::cast_from(3.0_f64);
    t35148
}
