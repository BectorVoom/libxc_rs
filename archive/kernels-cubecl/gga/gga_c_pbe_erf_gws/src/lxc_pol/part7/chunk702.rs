//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 702/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk702<F: Float>(t169: F, t242: F, t5697: F, t535: F, t784: F, t1339: F, t700: F, t1354: F, t299: F, t1343: F, t1383: F, t766: F) -> (F, F, F, F, F, F, F, F) {
    let t5700 = F::cast_from(0.5188034422540342311e0_f64) * t169 * t5697 * t242;
    let t5701 = t784 * t535;
    let t5703 = t169 * t5701 * t242;
    let t5707 = F::cast_from(0.42447554366239164361e0_f64) * t169 * t1339 * t700;
    let t5708 = t299 * t1354;
    let t5710 = t169 * t5708 * t242;
    let t5713 = t169 * t1343 * t700;
    let t5717 = F::cast_from(0.15917832887339686635e0_f64) * t169 * t766 * t1383;
    (t5700, t5701, t5703, t5707, t5708, t5710, t5713, t5717)
}
