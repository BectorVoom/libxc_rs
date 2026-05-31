//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2473/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2473<F: Float>(t3863: F, t5567: F, t3857: F, t2608: F, t512: F, t5566: F, t1856: F, t9544: F, t46975: F, t46979: F, t13597: F, t2516: F) -> (F, F, F, F, F, F, F) {
    let t48234 = F::cast_from(96.0_f64) * t3863 * t5567;
    let t48235 = t3857 * t5567;
    let t48236 = F::cast_from(60.0_f64) * t48235;
    let t48240 = t512 * t5566 * t2608;
    let t48241 = F::cast_from(3.0_f64) * t48240;
    let t48243 = t512 * t1856 * t9544;
    let t48244 = F::cast_from(240.0_f64) * t46975;
    let t48248 = F::cast_from(96.0_f64) * t46979;
    let t48255 = t13597 * t2516;
    (t48234, t48236, t48241, t48243, t48244, t48248, t48255)
}
