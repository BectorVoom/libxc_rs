//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1363/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1363<F: Float>(t1656: F, t18967: F, t20155: F, t219: F, t1266: F, t13047: F, t13055: F, t13109: F, t1639: F, t1657: F, t18483: F, t18490: F, t18496: F, t18499: F, t18947: F, t20157: F, t20183: F, t20190: F, t20200: F, t20206: F, t20211: F, t3366: F, t3367: F, t3384: F, t3385: F, t5739: F, t5740: F, t5921: F, t5925: F, t5930: F, t60653: F, t60778: F, t62453: F, t6419: F, t6424: F, t6425: F, t65667: F, t65818: F) -> F {
    let t67061 = t18967 * t1656;
    let t67083 = t20155 * t219;
    let t67109 = F::cast_from(2.0_f64) * t18483 * t20211 + F::cast_from(12.0_f64) * t60653 * t67061 * t18499 - F::cast_from(6.0_f64) * t5739 * t18490 * t6419 * t3366 - F::cast_from(6.0_f64) * t5739 * t18490 * t6424 * t3384 - F::cast_from(4.0_f64) * t18496 * t18967 * t1639 * t18499 - t5921 * t13109 + F::cast_from(2.0_f64) * t65667 * t5930 + F::cast_from(2.0_f64) * t5921 * t13055 - t62453 * t1657 - F::cast_from(2.0_f64) * t67083 * t1266 + F::cast_from(2.0_f64) * t60778 * t6425 + F::cast_from(4.0_f64) * t18483 * t20206 + F::cast_from(8.0_f64) * t18496 * t20190 * t1639 * t65818 + F::cast_from(2.0_f64) * t20157 * t3367 + F::cast_from(2.0_f64) * t18483 * t20200 + F::cast_from(2.0_f64) * t5739 * t5740 * t18947 * t1656 + F::cast_from(4.0_f64) * t18483 * t20183 - t20157 * t3385 - F::cast_from(6.0_f64) * t5921 * t13047 + F::cast_from(4.0_f64) * t65667 * t5925;
    t67109
}
