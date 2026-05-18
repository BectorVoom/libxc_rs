//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1100/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1100<F: Float>(t23405: F, t35022: F, t1349: F, t34962: F, t376: F, t34799: F, t104205: F, t1058: F, t138415: F, t138533: F, t138538: F, t1389: F, t1969: F, t26535: F, t26551: F, t26581: F, t26769: F, t28: F, t32870: F, t32967: F, t35012: F, t5766: F, t5772: F, t5778: F, t7309: F, t7342: F, t925: F) -> F {
    let t147160 = t23405 * t35022;
    let t147184 = t1349 * t376 * t34962;
    let t147191 = t1349 * t376 * t34799;
    let t147195 = -t147160 / F::new(27.0) - t138533 / F::new(18.0) - t7309 * t26535 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1349 * t28 * t5778 * t104205 + t1349 * t28 * t26769 * t1389 / F::new(3.0) + t1349 * t28 * t32967 * t26551 + t138538 / F::new(54.0) + t26581 * t7342 / F::new(6.0) + t1349 * t28 * t32870 * t1058 / F::new(6.0) - t147184 / F::new(18.0) - t5772 * t1969 * t138415 * t925 / F::new(9.0) - t147191 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t5766 * t35012;
    t147195
}
