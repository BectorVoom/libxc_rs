//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 821/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk821<F: Float>(t32944: F, t32960: F, t32977: F, t32949: F, t32954: F, t32957: F, t32965: F, t32970: F, t32974: F, t32982: F, t32986: F, t32990: F) -> (F, F, F, F) {
    let t33106 = t32944 / F::cast_from(18.0_f64);
    let t33110 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t32960;
    let t33114 = t32977 / F::cast_from(9.0_f64);
    let t33118 = t33106 + t32949 / F::cast_from(18.0_f64) + t32954 / F::cast_from(3.0_f64) - t32957 / F::cast_from(6.0_f64) - t33110 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t32965 - F::cast_from(2.0_f64) * t32970 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t32974 + t33114 + t32982 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t32986 - t32990 / F::cast_from(3.0_f64);
    (t33106, t33110, t33114, t33118)
}
