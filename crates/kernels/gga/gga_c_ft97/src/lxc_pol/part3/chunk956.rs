//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 956/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk956<F: Float>(t18587: F, t258: F, t18217: F, t18221: F, t18233: F, t18387: F, t18392: F, t18398: F, t18492: F, t18627: F, t18659: F, t18750: F) -> F {
    let t18760 = t18587 * t258;
    let t18772 = F::cast_from(2.0_f64) * t18760 - F::cast_from(2.0_f64) * t18392 - F::cast_from(4.0_f64) * t18233 + F::cast_from(8.0_f64) * t18659 - F::cast_from(4.0_f64) * t18221 + F::cast_from(4.0_f64) * t18627 - F::cast_from(12.0_f64) * t18217 + F::cast_from(8.0_f64) * t18750 - F::cast_from(2.0_f64) * t18398 + F::cast_from(4.0_f64) * t18492 - F::cast_from(2.0_f64) * t18387;
    t18772
}
