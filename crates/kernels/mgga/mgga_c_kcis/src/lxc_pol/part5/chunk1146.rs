//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1146/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1146<F: Float>(t1003: F, t19267: F, t417: F, t4966: F, t4972: F, t6517: F, t9959: F, t991: F, t2880: F, t6525: F, t6529: F, t19256: F, t19260: F, t19264: F, t2872: F, t6518: F, t6526: F, t6530: F, t6535: F, t984: F, t9970: F) -> F {
    let t19268 = t19267 * t1003;
    let t19269 = t417 * t19268;
    let t19272 = t4966 * t4972;
    let t19273 = t417 * t19272;
    let t19278 = t9959 * t6517;
    let t19279 = t991 * t19278;
    let t19283 = t2880 * t6525;
    let t19284 = t991 * t19283;
    let t19288 = t2880 * t6529;
    let t19289 = t991 * t19288;
    let t19292 = -t984 * t6535 / F::new(18.0) + t19256 / F::new(144.0) + t991 * t19260 / F::new(48.0) + t991 * t19264 / F::new(288.0) - t991 * t19269 / F::new(16.0) + t991 * t19273 / F::new(24.0) - t2872 * t6518 / F::new(81.0) + t19279 / F::new(648.0) + t2872 * t6526 / F::new(54.0) - t19284 / F::new(432.0) - t2872 * t6530 / F::new(108.0) + t19289 / F::new(864.0) + t9970 / F::new(162.0);
    t19292
}
