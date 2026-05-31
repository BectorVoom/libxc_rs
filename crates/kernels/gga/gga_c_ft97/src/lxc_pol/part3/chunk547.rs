//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 547/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk547<F: Float>(t4370: F, t898: F, t900: F, t2265: F, t2912: F, t2913: F, t2915: F, t3628: F, t4332: F, t4335: F, t4339: F, t4343: F, t4347: F, t4350: F, t4354: F, t4359: F, t631: F) -> (F, F) {
    let t4372 = t898 * t900 * t4370;
    let t4375 = -t2912 - t2913 / F::cast_from(9.0_f64) - t2915 / F::cast_from(3.0_f64) - t4332 / F::cast_from(9.0_f64) + t2265 * t4335 / F::cast_from(18.0_f64) - t2265 * t4339 / F::cast_from(3.0_f64) - t2265 * t4343 / F::cast_from(3.0_f64) - t3628 * t4347 / F::cast_from(3.0_f64) - t4350 / F::cast_from(3.0_f64) - t2265 * t4354 / F::cast_from(3.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t631 * t4359 + t631 * t4372 / F::cast_from(2.0_f64);
    (t4372, t4375)
}
