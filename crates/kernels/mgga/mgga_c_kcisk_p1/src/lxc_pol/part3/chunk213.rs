//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 213/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk213<F: Float>(t891: F, t896: F, t898: F, t102: F, t142: F, t157: F, t67: F, t69: F, t857: F, t863: F, t881: F, t884: F, t889: F, t89: F) -> (F, F) {
    let t899 = t891 * t896 * t898;
    let t903 = t67 * (F::cast_from(0.11073577833333333333e-2_f64) * t857 * t157 * t89 + F::new(1.0) * t863 * t881 - F::cast_from(0.18311555036753159941e-3_f64) * t884 * t69 * t142 * t102 - F::cast_from(0.58482233974552040708e0_f64) * t889 * t899);
    (t899, t903)
}
