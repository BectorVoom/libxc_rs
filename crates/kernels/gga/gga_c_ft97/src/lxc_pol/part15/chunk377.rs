//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 377/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk377<F: Float>(t2434: F, t304: F, t305: F, t1771: F, t303: F, t2344: F, t295: F) -> (F, F, F, F) {
    let t2730 = F::cast_from(0.11113000182098765433e-1_f64) * t2434;
    let t2755 = F::new(1.0) / t305 / t304;
    let t2761 = F::new(4.0) / F::new(9.0) * t1771 * t303;
    let t2766 = t2344 * t295;
    (t2730, t2755, t2761, t2766)
}
