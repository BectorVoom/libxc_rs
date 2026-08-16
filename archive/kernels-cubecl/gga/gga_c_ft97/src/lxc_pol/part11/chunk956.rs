//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 956/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk956<F: Float>(t137: F, t8906: F, t135: F, t2059: F, t2071: F, t2030: F, t2035: F, t2037: F, t1701: F, t37614: F, t538: F, t554: F) -> (F, F, F, F, F, F, F) {
    let t39801 = F::cast_from(1.0_f64) / t8906 / t137;
    let t39802 = t135 * t39801;
    let t39803 = t2059 * t2059;
    let t39807 = t2071 * t2071;
    let t39813 = t2030 * t2030;
    let t39818 = t2035 * t2037 * t2071;
    let t39824 = t1701 * t37614 * t538;
    let t39828 = t1701 * t37614 * t554;
    (t39802, t39803, t39807, t39813, t39818, t39824, t39828)
}
