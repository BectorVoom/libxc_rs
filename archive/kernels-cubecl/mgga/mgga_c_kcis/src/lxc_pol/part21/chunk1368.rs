//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1368/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1368<F: Float>(t7772: F, t96940: F, t1250: F, t15198: F, t251: F, t96123: F, t96137: F, t7775: F, t7796: F, t8087: F, t92590: F, t93047: F, t93053: F, t96121: F, t96127: F, t96130: F, t96133: F) -> (F, F) {
    let t97265 = F::cast_from(0.30918233506944444444e-4_f64) * t7772 * t96940;
    let t97267 = t15198 * t251 * t1250;
    let t97273 = F::cast_from(0.23214722222222222222e-2_f64) * t96123;
    let t97281 = F::cast_from(0.23214722222222222222e-2_f64) * t96137;
    let t97282 = t97265 + F::cast_from(0.69505208333333333334e-3_f64) * t97267 * t7796 + F::cast_from(0.69505208333333333334e-3_f64) * t97267 * t7775 - F::cast_from(0.25794135802469135802e-3_f64) * t96121 - t97273 + F::cast_from(0.34822083333333333332e-2_f64) * t96127 + F::cast_from(0.34752604166666666667e-3_f64) * t92590 * t8087 - F::cast_from(0.46377350260416666666e-4_f64) * t93047 + F::cast_from(0.15459116753472222222e-4_f64) * t93053 - F::cast_from(0.17411041666666666666e-2_f64) * t96130 - F::cast_from(0.17024129629629629629e-1_f64) * t96133 - t97281;
    (t97267, t97282)
}
