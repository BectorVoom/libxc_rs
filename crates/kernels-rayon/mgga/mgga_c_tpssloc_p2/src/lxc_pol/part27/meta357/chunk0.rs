//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1477/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1477(t2644: f64, t820: f64, t1509: f64, t828: f64, t2647: f64, t2632: f64, t776: f64, t1500: f64, t2693: f64, t4163: f64, t838: f64, t120: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13222 = t2644 * t820;
    let t13223 = t1509 * t828;
    let t13224 = t13223 * t2647;
    let t13225 = t13222 * t13224;
    let t13228 = t1509 * t2632;
    let t13229 = t828 * t776;
    let t13230 = t13228 * t13229;
    let t13231 = t13222 * t13230;
    let t13234 = t1500 * t2693;
    let t13237 = 7.0_f64 / 2304.0_f64 * t4163 * t838;
    let t13242 = t120 * t4233;
    (t13223, t13224, t13225, t13228, t13229, t13230, t13231, t13234, t13237, t13242)
}
