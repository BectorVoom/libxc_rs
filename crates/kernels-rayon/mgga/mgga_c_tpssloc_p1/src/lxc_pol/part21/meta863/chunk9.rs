//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3150/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3150(t3030: f64, t6150: f64, t3609: f64, t3623: f64, t5011: f64, t491: f64, t63280: f64, t64446: f64, t64454: f64, t64456: f64, t64458: f64, t64460: f64, t64462: f64, t64464: f64, t64466: f64, t64470: f64, t64472: f64, t64475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65253 = t6150 * t3030;
    let t65254 = t65253 * t3609;
    let t65262 = t65253 * t3623;
    let t65264 = t5011 * t5011;
    let t65265 = t491 * t65264;
    let t65278 = t64446 - t64454 - t64456 - t64458 - t64460 - t64462 - t64464 + t64466 + t64470 + t63280 + t64472 + t64475;
    (t65253, t65254, t65262, t65264, t65265, t65278)
}
