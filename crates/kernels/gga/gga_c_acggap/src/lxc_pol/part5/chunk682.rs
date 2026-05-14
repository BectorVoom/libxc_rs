//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 682/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk682<F: Float>(t1165: F, t3176: F, t4267: F, t1017: F, t960: F, t1322: F, t922: F, t1315: F, t3621: F, t4417: F, t1137: F, t1319: F, t1524: F, t174: F, t301: F, t1586: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5157 = t1165 * t4267 * t3176;
    let t5160 = t4267 * t1017;
    let t5161 = t960 * t5160;
    let t5164 = t1322 * t922;
    let t5165 = t960 * t5164;
    let t5169 = 7.0 / 24.0 * t3621 * t1315;
    let t5170 = t4417 * t1017;
    let t5171 = t960 * t5170;
    let t5175 = 7.0 / 72.0 * t1137 * t1319;
    let t5182 = t174 * t1524;
    let t5183 = t5182 * t301;
    let t5184 = t960 * t5183;
    let t5187 = t1586 * t372;
    (t5157, t5160, t5161, t5164, t5165, t5169, t5170, t5171, t5175, t5183, t5184, t5187)
}
