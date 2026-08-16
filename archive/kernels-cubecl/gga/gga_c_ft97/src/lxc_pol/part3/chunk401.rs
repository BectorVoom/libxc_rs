//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 401/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk401<F: Float>(t2724: F, t287: F, t2434: F, t863: F, t870: F, t304: F, t305: F, t1771: F, t303: F, t1775: F, t849: F, t458: F, t854: F) -> (F, F, F, F, F, F, F) {
    let t2725 = t287 * t2724;
    let t2730 = F::cast_from(0.11113000182098765433e-1_f64) * t2434;
    let t2749 = t863 * t870;
    let t2755 = F::cast_from(1.0_f64) / t305 / t304;
    let t2761 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1771 * t303;
    let t2762 = t1775 * t849;
    let t2764 = t458 * t854;
    (t2725, t2730, t2749, t2755, t2761, t2762, t2764)
}
