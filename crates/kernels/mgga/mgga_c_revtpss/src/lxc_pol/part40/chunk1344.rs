//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1344/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1344<F: Float>(t31463: F, t575: F, t1464: F, t8416: F, t1455: F, t8433: F, t116: F, t31451: F, t117338: F, t118085: F, t1459: F, t1461: F, t1518: F, t18190: F, t1916: F, t1918: F, t2209: F, t2327: F, t2371: F, t31217: F, t31234: F, t31241: F, t31475: F, t31497: F, t31500: F, t31505: F, t4158: F, t4165: F, t4292: F, t572: F, t573: F, t5795: F, t5802: F, t670: F, t8336: F, t8343: F, t8346: F, t8406: F, t8421: F, t8430: F) -> (F, F, F, F) {
    let t118106 = 2.0 * t31463 * t575;
    let t118108 = 2.0 * t8416 * t1464;
    let t118110 = 2.0 * t1455 * t8433;
    let t118137 = t116 * t31451;
    let t118154 = 3.0 * t1916 * t31241 + 6.0 * t572 * t117338 * t1518 + 12.0 * t572 * t31234 * t4292 + 3.0 * t31217 * t1918 + 6.0 * t5795 * t8346 + 6.0 * t572 * t2327 * t8406 + 12.0 * t1459 * t31497 + 6.0 * t31475 * t1461 + 3.0 * t4158 * t8430 + 12.0 * t5795 * t8343 + 12.0 * t572 * t118137 * t670 + 6.0 * t572 * t31505 * t2371 + 12.0 * t1459 * t31500 + 3.0 * t18190 * t2209 + param_d * t118085 * t573 + 12.0 * t8336 * t5802 + 3.0 * t8421 * t4165;
    (t118106, t118108, t118110, t118154)
}
