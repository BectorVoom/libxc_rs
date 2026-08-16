//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 441/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk441(t206: f64, t673: f64, t207: f64, t664: f64, t1650: f64, t1662: f64, t1667: f64, t1693: f64, t1707: f64, t1721: f64, t1917: f64, t1923: f64, t220: f64, t390: f64, t741: f64, t750: f64) -> (f64, f64, f64, f64) {
    let t1931 = t673 * t206;
    let t1932 = t207 * t664;
    let t1933 = t1931 * t1932;
    let t1936 = 0.5848223622634646207e0_f64 * t220 * t1917 + 0.19263893255070628431e1_f64 * t1707 + 0.65061487801810439052e-1_f64 * t1721 - 0.1301229756036208781e0_f64 * t1693 - 0.41096e0_f64 * t673 * t1923 * t207 + t1650 + 0.21687162600603479684e-1_f64 * t390 * t741 - 0.32106488758451047386e0_f64 * t390 * t750 - t1662 + t1667 + 0.68493333333333333332e-1_f64 * t390 * t1933;
    (t1931, t1932, t1933, t1936)
}
