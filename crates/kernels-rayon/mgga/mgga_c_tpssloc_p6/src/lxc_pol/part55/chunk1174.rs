//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1174/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1174(t118466: f64, t22960: f64, t7537: f64, t857: f64, t22986: f64, t23270: f64, t776: f64, t32814: f64, t82159: f64, t32815: f64, t81591: f64, t112899: f64, t1888: f64, t25045: f64) -> (f64, f64, f64, f64, f64) {
    let t118467 = t22960 * t118466;
    let t118472 = t857 * t7537;
    let t118476 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t118472 * t776;
    let t118479 = 0.3289868133696452873e-1_f64 * t22986 * t82159 * t32814;
    let t118480 = t81591 * t32815;
    let t118481 = 0.76763589786250567037e-1_f64 * t118480;
    let t118484 = 0.3289868133696452873e-1_f64 * t1888 * t112899 * t25045;
    (t118467, t118476, t118479, t118481, t118484)
}
