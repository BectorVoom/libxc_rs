//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 824/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk824<F: Float>(t13523: F, t13526: F, t13530: F, t13533: F, t13536: F, t13540: F, t13543: F, t13546: F, t13549: F, t13552: F, t13555: F, t1265: F, t4125: F, t1273: F, t4101: F, t373: F, t4128: F) -> (F, F, F, F) {
    let t13557 = -t13523 - 0.23744444444444444444e-1 * t13526 + 0.11872222222222222222e-1 * t13530 - 0.35616666666666666666e-1 * t13533 + 0.17808333333333333333e-1 * t13536 - 0.19787037037037037037e-1 * t13540 + 0.71233333333333333332e-1 * t13543 - 0.35616666666666666666e-1 * t13546 - 0.10685e0 * t13549 + 0.10685e0 * t13552 - 0.17808333333333333333e-1 * t13555;
    let t13561 = 1.0 / t4125 / t1265;
    let t13562 = t4101 * t1273;
    let t13565 = 1.0 / t4128 / t373;
    (t13557, t13561, t13562, t13565)
}
