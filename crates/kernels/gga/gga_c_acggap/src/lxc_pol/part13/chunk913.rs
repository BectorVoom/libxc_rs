//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 913/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk913<F: Float>(t30371: F, t5152: F, t5138: F, t8511: F, t30273: F, t30280: F, t5143: F, t31362: F, t8783: F, t1165: F, t20595: F, t604: F, t7337: F, t1426: F, t1579: F, t2085: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t34072 = t30371 * t5152;
    let t34074 = t8511 * t5138;
    let t34076 = 0.21437009059034868486e-3 * t30273;
    let t34077 = 0.28582678745379824648e-3 * t30280;
    let t34078 = t8511 * t5143;
    let t34081 = t31362 * t8783;
    let t34082 = 0.15724046144802076034e-2 * t34081;
    let t34085 = t7337 * t1165 * t604 * t20595;
    let t34089 = t598 * t1426 * t1579 * t2085;
    (t34072, t34074, t34076, t34077, t34078, t34082, t34085, t34089)
}
