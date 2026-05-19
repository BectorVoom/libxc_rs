//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1080/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1080<F: Float>(t46952: F, t493: F, t105: F, t492: F, t42687: F, t42689: F, t42691: F, t42694: F, t42695: F, t42698: F, t42700: F, t42703: F, t42706: F, t42708: F) -> (F, F) {
    let t46953 = t493 * t46952;
    let t46956 = F::cast_from(0.28455006635676149599e-1_f64) * t105 * t492 * t46953;
    let t46959 = -t46956 + t42687 - t42689 - t42691 - t42694 - F::cast_from(0.1138200265427045984e0_f64) * t42695 + t42698 - F::cast_from(0.85365019907028448797e-1_f64) * t42700 + t42703 + t42706 + t42708;
    (t46953, t46959)
}
