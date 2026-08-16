//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1368/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1368(t7772: f64, t96940: f64, t1250: f64, t15198: f64, t251: f64, t96123: f64, t96137: f64, t7775: f64, t7796: f64, t8087: f64, t92590: f64, t93047: f64, t93053: f64, t96121: f64, t96127: f64, t96130: f64, t96133: f64) -> (f64, f64) {
    let t97265 = 0.30918233506944444444e-4_f64 * t7772 * t96940;
    let t97267 = t15198 * t251 * t1250;
    let t97273 = 0.23214722222222222222e-2_f64 * t96123;
    let t97281 = 0.23214722222222222222e-2_f64 * t96137;
    let t97282 = t97265 + 0.69505208333333333334e-3_f64 * t97267 * t7796 + 0.69505208333333333334e-3_f64 * t97267 * t7775 - 0.25794135802469135802e-3_f64 * t96121 - t97273 + 0.34822083333333333332e-2_f64 * t96127 + 0.34752604166666666667e-3_f64 * t92590 * t8087 - 0.46377350260416666666e-4_f64 * t93047 + 0.15459116753472222222e-4_f64 * t93053 - 0.17411041666666666666e-2_f64 * t96130 - 0.17024129629629629629e-1_f64 * t96133 - t97281;
    (t97267, t97282)
}
