//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 747/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk747<F: Float>(t33452: F, t743: F, t1434: F, t193: F, t375: F, t7528: F, t89: F, t668: F, t7440: F) -> (F, F, F, F, F) {
    let t33453 = t743 * t33452;
    let t33455 = t1434 * t193 * t33453;
    let t33458 = t89 * t375 * t7528;
    let t33459 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33458;
    let t33460 = t7440 * t668;
    (t33453, t33455, t33458, t33459, t33460)
}
