//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1034/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1034<F: Float>(t19278: F, t991: F, t2880: F, t6525: F, t6529: F, t19256: F, t19260: F, t19264: F, t19269: F, t19273: F, t2872: F, t6518: F, t6526: F, t6530: F, t6535: F, t984: F, t9970: F) -> (F,) {
    let t19279 = t991 * t19278;
    let t19283 = t2880 * t6525;
    let t19284 = t991 * t19283;
    let t19288 = t2880 * t6529;
    let t19289 = t991 * t19288;
    let t19292 = -t984 * t6535 / 18.0 + t19256 / 144.0 + t991 * t19260 / 48.0 + t991 * t19264 / 288.0 - t991 * t19269 / 16.0 + t991 * t19273 / 24.0 - t2872 * t6518 / 81.0 + t19279 / 648.0 + t2872 * t6526 / 54.0 - t19284 / 432.0 - t2872 * t6530 / 108.0 + t19289 / 864.0 + t9970 / 162.0;
    (t19292,)
}
