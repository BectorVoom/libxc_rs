//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 655/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk655(t12161: f64, t836: f64, t568: f64, t739: f64, t531: f64, t808: f64, t314: f64, t313: f64, t3732: f64, t769: f64, t1628: f64, t3740: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12162 = t836 * t12161;
    let t12163 = t568 * t12162;
    let t12166 = t739 * t12161;
    let t12167 = t531 * t12166;
    let t12172 = t808 * t12161;
    let t12173 = t568 * t12172;
    let t12176 = t314 * t12161;
    let t12177 = t313 * t12176;
    let t12182 = t769 * t3732;
    let t12185 = t1628 * t3740;
    (t12163, t12166, t12167, t12173, t12176, t12177, t12182, t12185)
}
