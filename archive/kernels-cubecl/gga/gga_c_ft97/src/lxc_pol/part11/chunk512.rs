//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 512/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk512<F: Float>(t2951: F, t898: F, t900: F, t2265: F, t2912: F, t2913: F, t2915: F, t2920: F, t2925: F, t2930: F, t2934: F, t2941: F, t631: F) -> (F, F) {
    let t2953 = t898 * t900 * t2951;
    let t2956 = -t2912 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2913 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2915 + t631 * t2920 / F::cast_from(18.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t2925 - t631 * t2930 / F::cast_from(3.0_f64) + t631 * t2934 / F::cast_from(6.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t631 * t2941 + t631 * t2953 / F::cast_from(2.0_f64);
    (t2953, t2956)
}
