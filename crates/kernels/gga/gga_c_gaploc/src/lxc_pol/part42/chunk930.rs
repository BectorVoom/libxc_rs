//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 930/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk930<F: Float>(t10348: F, t11362: F, t11429: F, t1415: F, t7030: F, t13471: F, t7014: F, t2898: F, t44310: F, t900: F, t13415: F, t4950: F) -> (F, F, F, F, F) {
    let t46724 = F::cast_from(0.7150097990370085334e0_f64) * t11362 * t10348;
    let t46729 = t1415 * t11429 * t7030;
    let t46730 = F::cast_from(0.14896037479937677779e-1_f64) * t46729;
    let t46731 = t7014 * t13471;
    let t46732 = F::cast_from(0.19171462976960374838e0_f64) * t46731;
    let t46734 = t2898 * t900 * t44310;
    let t46735 = F::cast_from(0.29792074959875355558e-1_f64) * t46734;
    let t46740 = F::cast_from(0.71500979903700853338e0_f64) * t4950 * t13415;
    (t46724, t46730, t46732, t46735, t46740)
}
