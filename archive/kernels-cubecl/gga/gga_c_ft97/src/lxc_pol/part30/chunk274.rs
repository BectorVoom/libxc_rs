//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 274/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk274<F: Float>(t3803: F, t701: F, t2440: F, t420: F, t3691: F, t2320: F, t3700: F, t18: F, t704: F, t2248: F, t2435: F, t2437: F, t3796: F, t3800: F) -> (F, F, F, F, F) {
    let t3804 = t701 * t3803;
    let t3806 = t420 * t2440;
    let t3807 = t3806 * t3691;
    let t3808 = t701 * t3807;
    let t3810 = t2320 * t3700;
    let t3811 = t701 * t3810;
    let t3813 = t704 * t18;
    let t3814 = t2248 * t3813;
    let t3815 = t701 * t3814;
    let t3817 = -F::cast_from(0.17024962234567901235e-1_f64) * t3796 - F::cast_from(0.17024962234567901235e-1_f64) * t3800 - t2435 + F::cast_from(0.21281202793209876543e-2_f64) * t2437 + F::cast_from(0.21281202793209876543e-2_f64) * t3804 + F::cast_from(0.85124811172839506173e-2_f64) * t3808 - F::cast_from(0.12768721675925925926e-1_f64) * t3811 - F::cast_from(0.12768721675925925926e-1_f64) * t3815;
    (t3804, t3808, t3811, t3815, t3817)
}
