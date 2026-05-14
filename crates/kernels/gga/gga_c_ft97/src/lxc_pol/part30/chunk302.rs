//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 302/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk302<F: Float>(t4226: F, t845: F, t91: F, t4032: F, t4049: F, t2656: F, t2659: F, t2823: F, t4035: F, t4039: F, t4042: F, t4046: F, t4054: F, t4059: F, t4132: F, t4193: F) -> (F, F) {
    let t4228 = t91 * t845 * t4226;
    let t4230 = t4032 / 27.0;
    let t4235 = t4049 / 9.0;
    let t4239 = -t4193 / 12.0 + t4228 / 6.0 + t2823 + t2656 + t2659 + t4230 - 2.0 / 27.0 * t4035 + t4039 / 9.0 + 2.0 / 9.0 * t4042 + 2.0 / 9.0 * t4046 + t4235 + t4054 / 9.0 + 2.0 / 3.0 * t4059 - t4132 / 3.0;
    (t4228, t4239)
}
