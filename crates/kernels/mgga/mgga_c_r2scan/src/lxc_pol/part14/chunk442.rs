//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 442/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk442<F: Float>(t206: F, t673: F, t207: F, t664: F, t1650: F, t1662: F, t1667: F, t1693: F, t1707: F, t1721: F, t1917: F, t1923: F, t220: F, t390: F, t741: F, t750: F) -> (F, F, F, F) {
    let t1931 = t673 * t206;
    let t1932 = t207 * t664;
    let t1933 = t1931 * t1932;
    let t1936 = F::cast_from(0.5848223622634646207e0_f64) * t220 * t1917 + F::cast_from(0.19263893255070628431e1_f64) * t1707 + F::cast_from(0.65061487801810439052e-1_f64) * t1721 - F::cast_from(0.1301229756036208781e0_f64) * t1693 - F::cast_from(0.41096e0_f64) * t673 * t1923 * t207 + t1650 + F::cast_from(0.21687162600603479684e-1_f64) * t390 * t741 - F::cast_from(0.32106488758451047386e0_f64) * t390 * t750 - t1662 + t1667 + F::cast_from(0.68493333333333333332e-1_f64) * t390 * t1933;
    (t1931, t1932, t1933, t1936)
}
