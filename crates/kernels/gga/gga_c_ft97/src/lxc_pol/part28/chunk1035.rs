//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1035/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1035<F: Float>(t137105: F, t137108: F, t137110: F, t137124: F, t137131: F, t144989: F, t144994: F, t145001: F, t145004: F, t145008: F, t145012: F, t145017: F, t145022: F, t145025: F, t145028: F, t145032: F) -> F {
    let t145034 = t137105 - F::new(2.0) / F::new(3.0) * t137108 - t137110 / F::new(18.0) + F::new(2.0) * t144989 + F::new(4.0) * t144994 - t137124 / F::new(3.0) + t137131 / F::new(6.0) + F::new(2.0) * t145001 - F::new(2.0) / F::new(3.0) * t145004 - F::new(2.0) * t145008 - F::new(2.0) * t145012 + t145017 / F::new(4.0) + t145022 / F::new(4.0) - F::new(4.0) / F::new(3.0) * t145025 + F::new(2.0) * t145028 + F::new(3.0) / F::new(2.0) * t145032;
    t145034
}
