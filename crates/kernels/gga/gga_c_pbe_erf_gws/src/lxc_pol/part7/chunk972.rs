//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 972/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk972<F: Float>(t17877: F, t18008: F, t163: F, t169: F, t234: F, t922: F, t1354: F, t784: F, t1378: F, t1971: F, t4585: F, t5701: F) -> (F, F, F, F, F) {
    let t18009 = t17877 + t18008;
    let t18021 = F::cast_from(0.40978489723982440011e0_f64) * t169 * t922 * t234 * t163;
    let t18022 = t784 * t1354;
    let t18024 = t18022 * t1378 * t1971;
    let t18027 = t5701 * t4585 * t1971;
    (t18009, t18021, t18022, t18024, t18027)
}
