//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1022/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1022<F: Float>(t330: F, t4972: F, t829: F, t2894: F, t13462: F, t4939: F, t291: F, t9897: F, t13467: F, t2887: F, t736: F, t13516: F, t1662: F, t3040: F, t2909: F, t1003: F) -> (F, F, F, F, F, F, F) {
    let t14484 = t4972 * t330;
    let t14485 = t14484 * t829;
    let t14486 = t2894 * t14485;
    let t14489 = t4939 * t13462;
    let t14492 = t9897 * t291;
    let t14493 = t14492 * t13467;
    let t14496 = t736 * t2887;
    let t14497 = t14496 * t291;
    let t14498 = t14497 * t13516;
    let t14501 = t1662 * t3040;
    let t14502 = t2894 * t14501;
    let t14511 = t2909 * t4972;
    let t14512 = t14511 * t1003;
    (t14486, t14489, t14493, t14496, t14498, t14502, t14512)
}
