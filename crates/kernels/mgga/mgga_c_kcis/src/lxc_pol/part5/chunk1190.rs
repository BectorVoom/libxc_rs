//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1190/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1190<F: Float>(t19905: F, t389: F, t19756: F, t5181: F, t5180: F, t1195: F, t6727: F, t382: F, t3477: F, t6724: F, t14721: F, t1813: F) -> (F, F, F, F, F) {
    let t19906 = t19905 * t389;
    let t19908 = t5181 * t19756;
    let t19909 = t5180 * t19908;
    let t19911 = t1195 * t6727;
    let t19912 = t382 * t19911;
    let t19914 = t3477 * t6724;
    let t19916 = t14721 * t1813;
    (t19906, t19909, t19912, t19914, t19916)
}
