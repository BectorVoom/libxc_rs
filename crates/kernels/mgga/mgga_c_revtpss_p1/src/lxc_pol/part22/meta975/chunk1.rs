//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3277/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277<F: Float>(t2394: F, t40862: F, t40868: F, t51110: F, t51112: F, t51121: F, t51125: F, t51135: F, t5988: F, t62236: F, t62241: F, t62246: F, t62251: F, t800: F) -> F {
    let t62258 = -F::cast_from(0.50820002809285328225e-4_f64) * t62236 - F::cast_from(0.2032800112371413129e-3_f64) * t51110 - F::cast_from(0.25410001404642664112e-4_f64) * t62241 - F::cast_from(0.16006300097412701803e-1_f64) * t51112 + F::cast_from(0.45351183609335988443e-1_f64) * t51121 + F::cast_from(0.22866142996303859718e-3_f64) * t62246 + F::cast_from(0.2032800112371413129e-3_f64) * t51125 + F::cast_from(0.22866142996303859718e-3_f64) * t51135 - F::cast_from(0.18071592998981862717e-4_f64) * t62251 + F::new(5.0) / F::new(4.0) * t40868 * t800 * t5988 * t2394 + F::new(455.0) / F::new(324.0) * t40862;
    t62258
}
