//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1004/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1004<F: Float>(t4910: F, t733: F, t4913: F, t10114: F, t167: F, t1071: F, t2622: F, t4898: F, t738: F, t4901: F, t1072: F, t4547: F) -> (F, F, F, F, F, F, F) {
    let t13492 = F::new(0.18736e-1) * t733 * t4910;
    let t13493 = t733 * t4913;
    let t13499 = t10114 * t167;
    let t13501 = t2622 * t1071;
    let t13502 = t13501 * t167;
    let t13532 = t738 * t4898;
    let t13535 = F::cast_from(0.17611111111111111111e-2_f64) * t738 * t4901;
    let t13558 = F::cast_from(0.47822877300252710492e-1_f64) * t1072 * t4547;
    (t13492, t13493, t13499, t13502, t13532, t13535, t13558)
}
