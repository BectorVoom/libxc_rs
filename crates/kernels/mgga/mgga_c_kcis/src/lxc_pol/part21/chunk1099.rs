//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1099/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1099<F: Float>(t26987: F, t7784: F, t26960: F, t92850: F, t1014: F, t26833: F, t3245: F, t7723: F, t10470: F, t2180: F, t27077: F, t92751: F, t7732: F, t26720: F, t26800: F, t2822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93099 = t26987 * t7784;
    let t93134 = t26960 * t92850;
    let t93143 = t1014 * t26833;
    let t93145 = t3245 * t7723;
    let t93157 = t10470 * t2180;
    let t93158 = 0.51588271604938271604e-3 * t93157;
    let t93161 = t27077 * t92751;
    let t93163 = t3245 * t7732;
    let t93171 = t1014 * t26720;
    let t93173 = t2822 * t26800;
    (t93099, t93134, t93143, t93145, t93157, t93158, t93161, t93163, t93171, t93173)
}
