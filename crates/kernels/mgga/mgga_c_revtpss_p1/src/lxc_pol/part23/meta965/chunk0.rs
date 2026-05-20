//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3262/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262<F: Float>(t73352: F, t177: F, t22789: F, t762: F, t48227: F, t46973: F, t48243: F, t46977: F, t39483: F, t39520: F, t39528: F, t39531: F, t48224: F, t48226: F, t48234: F, t48236: F, t48241: F, t48244: F, t48248: F) -> (F, F, F, F, F, F, F) {
    let t85893 = F::cast_from(0.17544670867903938621e1_f64) * t73352;
    let t85895 = t22789 * t177 * t762;
    let t85896 = F::cast_from(0.5848223622634646207e0_f64) * t85895;
    let t85897 = F::new(180.0) * t48227;
    let t85898 = F::new(12.0) * t46973;
    let t85899 = F::new(3.0) * t48243;
    let t85900 = F::new(120.0) * t46977;
    let t85901 = -t85893 - t48224 - t39483 - t48226 + t39520 - t85896 + t85897 - t39528 - t85898 + t39531 + t48234 + t48236 + t48241 + t85899 - t48244 - t85900 + t48248;
    (t85893, t85896, t85897, t85898, t85899, t85900, t85901)
}
