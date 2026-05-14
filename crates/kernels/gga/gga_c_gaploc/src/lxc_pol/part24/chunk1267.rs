//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1267/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1267<F: Float>(t224: F, t32721: F, t32741: F, t33983: F, t35243: F, t11142: F, t617: F, t10289: F, t10299: F, t10293: F, t10302: F, t10625: F, t10292: F, t11143: F, t856: F, t2231: F, t31458: F, t31461: F, t31463: F, t31465: F, t31468: F, t31469: F, t31470: F, t31472: F, t31474: F, t31476: F, t31478: F, t31480: F, t31483: F, t31485: F, t32090: F, t32091: F, t32093: F, t32095: F, t3513: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35246 = t224 * (t32721 + t32741 + t33983 + t35243);
    let t35247 = t617 * t11142;
    let t35252 = 2.0 * t10289;
    let t35253 = 4.0 * t10299;
    let t35254 = 4.0 * t10293;
    let t35255 = 4.0 * t10302;
    let t35256 = 2.0 * t10625;
    let t35257 = 2.0 * t10292;
    let t35259 = 2.0 * t11143;
    let t39538 = t856 * t11142;
    let t39539 = t2231 * t3513 - t31458 - t31461 - t31463 + t31465 + t31468 - t31469 - t31470 + t31472 - t31474 + t31476 + t31478 + t31480 + t31483 - t31485 + t32090 - t32091 - t32093 + t32095 + t39538;
    (t35246, t35247, t35252, t35253, t35254, t35255, t35256, t35257, t35259, t39539)
}
