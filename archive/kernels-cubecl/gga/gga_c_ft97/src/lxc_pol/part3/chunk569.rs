//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 569/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk569<F: Float>(t103: F, t4545: F, t108: F, t4415: F, t4501: F, t4552: F, t4590: F, t4594: F, t4621: F, t88: F, t948: F, t984: F) -> (F, F) {
    let t4623 = t4545 * t103;
    let t4628 = -t108 * t4415 - t108 * t4501 - t4621 * t88 - F::cast_from(2.0_f64) * t948 * t984 + F::cast_from(4.0_f64) * t4552 - F::cast_from(2.0_f64) * t4590 - F::cast_from(4.0_f64) * t4594 + F::cast_from(2.0_f64) * t4623;
    (t4623, t4628)
}
