//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 761/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk761<F: Float>(t21655: F, t4265: F, t2881: F, t1091: F, t5424: F, t835: F, t1248: F, t5393: F, t2843: F, t296: F, t1212: F, t5309: F, t840: F, t21362: F, t319: F, t14946: F, t21947: F, t21951: F, t21955: F, t21960: F, t21964: F, t21967: F, t21971: F, t21975: F, t21984: F, t21987: F, t21991: F, t21994: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22397 = t4265 * t21655;
    let t22398 = t2881 * t22397;
    let t22402 = t835 * t5424 * t1091;
    let t22405 = t1248 * t5393;
    let t22406 = t2843 * t22405;
    let t22407 = t296 * t22406;
    let t22410 = t5309 * t1212;
    let t22412 = t840 * t2843 * t22410;
    let t22416 = t835 * t319 * t21362;
    let t22432 = 2.0 / 3.0 * t21994 + t21971 / 3.0 + t21975 / 3.0 + 2.0 / 9.0 * t21960 - 2.0 / 9.0 * t21967 - 2.0 / 3.0 * t21947 - 2.0 / 3.0 * t21951 - 10.0 / 81.0 * t21955 + 4.0 / 9.0 * t21964 + 2.0 * t21984 - t21987 / 9.0 - 2.0 / 3.0 * t21991 - t14946;
    (t22397, t22398, t22402, t22405, t22406, t22407, t22410, t22412, t22416, t22432)
}
