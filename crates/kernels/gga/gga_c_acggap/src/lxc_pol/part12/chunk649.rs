//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 649/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk649<F: Float>(t1165: F, t3176: F, t4267: F, t1017: F, t960: F, t1322: F, t922: F, t1315: F, t3621: F, t4417: F, t1137: F, t1319: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5157 = t1165 * t4267 * t3176;
    let t5160 = t4267 * t1017;
    let t5161 = t960 * t5160;
    let t5164 = t1322 * t922;
    let t5165 = t960 * t5164;
    let t5169 = F::new(7.0) / F::new(24.0) * t3621 * t1315;
    let t5170 = t4417 * t1017;
    let t5171 = t960 * t5170;
    let t5175 = F::new(7.0) / F::new(72.0) * t1137 * t1319;
    (t5157, t5160, t5161, t5164, t5165, t5169, t5170, t5171, t5175)
}
