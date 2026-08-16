//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 994/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk994<F: Float>(t13086: F, t64: F, t10657: F, t871: F, t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t2508: F, t2717: F, t3433: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43071 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13086 * t64;
    let t43072 = t10657 * t871;
    let t43075 = F::cast_from(7.0_f64) / F::cast_from(512.0_f64) * t40612;
    let t43076 = F::cast_from(63.0_f64) / F::cast_from(16384.0_f64) * t40614;
    let t43077 = F::cast_from(63.0_f64) / F::cast_from(1048576.0_f64) * t40620;
    let t43078 = F::cast_from(21.0_f64) / F::cast_from(1048576.0_f64) * t40630;
    let t43079 = F::cast_from(21.0_f64) / F::cast_from(16384.0_f64) * t40632;
    let t43080 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t40634;
    let t43087 = t2508 * t2717 * t3433;
    (t43071, t43072, t43075, t43076, t43077, t43078, t43079, t43080, t43087)
}
