//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 878/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk878<F: Float>(t11167: F, t11177: F, t12216: F, t12217: F, t16832: F, t16842: F, t16845: F, t17666: F, t3056: F, t3359: F, t383: F, t7946: F, t8698: F) -> F {
    let t17667 = F::new(0.1760655e0) * t16832 * t383 - F::new(0.234754e0) * t3359 * t3056 - F::new(0.117377e0) * t16842 * t383 + F::new(0.234754e0) * t16845 - t8698 - F::cast_from(0.6419148148148148148e-1_f64) * t7946 - F::cast_from(0.12838296296296296296e0_f64) * t11167 + t12217 - t12216 + F::cast_from(0.19257444444444444444e0_f64) * t11177 + t17666;
    t17667
}
