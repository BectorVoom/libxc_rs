//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1252/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1252<F: Float>(t654: F, t9206: F, t20: F, t2801: F, t2029: F, t9226: F, t2021: F, t1586: F, t24561: F, t2805: F) -> (F, F, F, F, F, F, F, F) {
    let t35461 = t9206 * t654;
    let t35462 = t35461 * t20;
    let t35463 = t2801 * t35462;
    let t35467 = t2029 * t9226;
    let t35468 = t2021 * t35467;
    let t35469 = t1586 * t35468;
    let t35475 = t2805 * t24561;
    let t35476 = t1586 * t35475;
    (t35461, t35462, t35463, t35467, t35468, t35469, t35475, t35476)
}
