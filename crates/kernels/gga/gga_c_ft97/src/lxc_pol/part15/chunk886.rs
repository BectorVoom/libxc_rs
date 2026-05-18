//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 886/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk886<F: Float>(t191: F, t33828: F, t10: F, t11175: F, t296: F, t190: F, t2680: F, t305: F, t36452: F, t37991: F, t11176: F, t303: F) -> (F, F, F, F, F) {
    let t43524 = t191 * t33828;
    let t43537 = t10 * t11175 * t296;
    let t43538 = F::new(280.0) / F::new(243.0) * t43537;
    let t43548 = F::new(1.0) / t305 / t37991 / t190 / t2680 / t36452 / F::new(96.0);
    let t43574 = F::new(280.0) / F::new(81.0) * t11176 * t303;
    (t43524, t43537, t43538, t43548, t43574)
}
