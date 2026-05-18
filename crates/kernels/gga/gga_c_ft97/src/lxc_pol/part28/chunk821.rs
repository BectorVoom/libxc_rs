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
    let t33106 = t32944 / F::new(18.0);
    let t33110 = F::new(2.0) / F::new(9.0) * t32960;
    let t33114 = t32977 / F::new(9.0);
    let t33118 = t33106 + t32949 / F::new(18.0) + t32954 / F::new(3.0) - t32957 / F::new(6.0) - t33110 - F::new(2.0) / F::new(9.0) * t32965 - F::new(2.0) * t32970 + F::new(4.0) / F::new(3.0) * t32974 + t33114 + t32982 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t32986 - t32990 / F::new(3.0);
    (t33106, t33110, t33114, t33118)
}
