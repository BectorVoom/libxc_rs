//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 725/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk725(t12452: f64, t12456: f64, t12924: f64, t12928: f64, t12929: f64, t12930: f64, t12935: f64, t12936: f64, t12937: f64, t12941: f64, t13780: f64, t13783: f64) -> f64 {
    let t14463 = t12924 - t12928 - t12929 + t12930 - 0.89376224879626066675e-1_f64 * t12452 + 0.59584149919750711115e-1_f64 * t12456 - 0.38342925953920749676e0_f64 * t13780 + 0.38342925953920749676e0_f64 * t13783 - t12935 + t12936 + t12937 - t12941;
    t14463
}
