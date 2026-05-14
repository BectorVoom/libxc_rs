//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 966/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk966<F: Float>(t3077: F, t4163: F, t1035: F, t1647: F, t3044: F, t1655: F, t848: F, t3645: F, t553: F, t4150: F, t1160: F, t5315: F, t930: F, t159: F, t322: F, t381: F, t5299: F) -> (F, F, F, F, F, F, F) {
    let t19060 = t3077 * t4163;
    let t19074 = t1035 * t1647 * t3044;
    let t19082 = t848 * t1655;
    let t19090 = t3645 * t553;
    let t19095 = t3077 * t4150;
    let t19098 = t1160 * t5315 * t930;
    let t19103 = t381 * t159 * t5299 * t322;
    (t19060, t19074, t19082, t19090, t19095, t19098, t19103)
}
